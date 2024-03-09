package core

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"log"
	"math/rand"
	"net"
	"strconv"
	"strings"
)

// Lookup returns the IP address for the given hostname
func Lookup(hostname string) string {
	ip := ROOT_SERVER
	recordType := RECORD_TYPE_A
	hostname = strings.TrimSuffix(hostname, ".")
	for {
		fmt.Printf("asking %s for %s\n", ip, hostname)
		packet := sendQuery(ip, hostname, uint16(recordType))

		// check for a direct answer
		for _, ans := range packet.Answers {
			if ans.RecordType == RECORD_TYPE_A {
				return parseIP(ans.Data)
			}
		}

		// all right, see if we have a nameserver we can hit?
		weHaveNSWithARecord := false
		for _, add := range packet.Additionals {
			if add.RecordType == RECORD_TYPE_A {
				ip = parseIP(add.Data)
				weHaveNSWithARecord = true
				break
			}
		}

		if weHaveNSWithARecord {
			continue
		}

		// we have a name server domain, get it's IP first
		for _, auth := range packet.Authorities {
			if auth.RecordType == RECORD_TYPE_NS && string(auth.Data) != "" {
				ip = Lookup(string(auth.Data))
				break
			}
		}
	}
}

func sendQuery(ip string, hostname string, recordType uint16) *Packet {
	conn, err := net.Dial("udp", fmt.Sprintf("%s:53", ip))
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()
	_, err = conn.Write(buildQuery(hostname, recordType))
	if err != nil {
		panic(err)
	}
	response := make([]byte, MAX_DNS_PACKET_SIZE_IN_BYTES)
	_, err = conn.Read(response)
	if err != nil {
		log.Fatal(err)
	}
	return parsePacket(response)
}

func encodeHostname(hostname string) []byte {
	var result bytes.Buffer
	parts := strings.Split(hostname, ".")
	for _, part := range parts {
		result.WriteByte(byte(len(part)))
		result.Write([]byte(part))
	}
	result.WriteByte(byte(0))
	return result.Bytes()
}

func uint16ToBytes(value uint16) []byte {
	res := make([]byte, 2)
	binary.BigEndian.PutUint16(res, value)
	return res
}

func headerToBytes(header Header) []byte {
	var result bytes.Buffer
	result.Write(uint16ToBytes(header.ID))
	result.Write(uint16ToBytes(header.Flags))
	result.Write(uint16ToBytes(header.NumberOfQuestions))
	result.Write(uint16ToBytes(header.NumberOfAnswers))
	result.Write(uint16ToBytes(header.NumberOfAuthorities))
	result.Write(uint16ToBytes(header.NumberOfAdditionals))
	return result.Bytes()
}

func questionToBytes(question Question) []byte {
	var result bytes.Buffer
	result.Write(question.Name)
	result.Write(uint16ToBytes(question.ClassType))
	result.Write(uint16ToBytes(question.RecordType))
	return result.Bytes()
}

func buildQuery(hostname string, recordType uint16) []byte {
	var query bytes.Buffer
	header := newHeader(uint16(rand.Int()), RECURSION_NOT_DESIRED, DEFAULT_NUMBER_OF_QUESTIONS)
	question := newQuestion(encodeHostname(hostname), recordType, CLASS_TYPE_IN)
	query.Write(headerToBytes(*header))
	query.Write(questionToBytes(*question))
	return query.Bytes()
}

func parseHeader(reader *bytes.Reader) *Header {
	var header Header
	err := binary.Read(reader, binary.BigEndian, &header)
	if err != nil {
		log.Fatal(err)
	}
	return &header
}

func decodeCompressedName(length byte, reader *bytes.Reader) []byte {
	b, _ := reader.ReadByte()
	pointerBytes := []byte{length & 0b0011_1111, b}
	pointer := binary.BigEndian.Uint16(pointerBytes)
	currentPos, err := reader.Seek(0, io.SeekCurrent)
	if err != nil {
		log.Fatal(err)
	}
	_, err = reader.Seek(int64(pointer), io.SeekStart)
	if err != nil {
		log.Fatal(err)
	}
	result := decodeName(reader)
	_, err = reader.Seek(currentPos, io.SeekStart)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func decodeName(reader *bytes.Reader) []byte {
	var name []byte
	for {
		length, _ := reader.ReadByte()
		if length == 0 {
			break
		}
		if length&0b1100_0000 != 0 {
			name = append(name, decodeCompressedName(length, reader)...)
			// add a period "."
			name = append(name, 46)
			break
		} else {
			part := make([]byte, length)
			_, err := io.ReadFull(reader, part)
			if err != nil {
				log.Fatal(err)
			}
			name = append(name, part...)
			// add a period "."
			name = append(name, 46)
		}
	}
	return name
}

func parseQuestion(reader *bytes.Reader) *Question {
	var internalQ internalQuestion
	name := decodeName(reader)
	err := binary.Read(reader, binary.BigEndian, &internalQ)
	if err != nil {
		log.Fatal(err)
	}
	return &Question{
		Name:       name,
		RecordType: internalQ.RecordType,
		ClassType:  internalQ.ClassType,
	}
}

func parseRecord(reader *bytes.Reader) *Record {
	var internalR internalRecord
	name := decodeName(reader)
	err := binary.Read(reader, binary.BigEndian, &internalR)
	if err != nil {
		log.Fatal(err)
	}
	var data = make([]byte, internalR.DataSize)
	if internalR.RecordType == RECORD_TYPE_NS {
		data = decodeName(reader)
	} else {
		_, err = io.ReadFull(reader, data)
		if err != nil {
			log.Fatal(err)
		}
	}
	return &Record{
		Name:       name,
		RecordType: internalR.RecordType,
		ClassType:  internalR.ClassType,
		TTL:        internalR.TTL,
		Data:       data,
	}
}

func parseNextNSegmentsAsRecords(reader *bytes.Reader, n uint16) []Record {
	records := make([]Record, n)
	for i := 0; i < int(n); i++ {
		records[i] = *parseRecord(reader)
	}
	return records
}

func parsePacket(response []byte) *Packet {
	reader := bytes.NewReader(response)
	header := parseHeader(reader)
	questions := make([]Question, header.NumberOfQuestions)
	for i := 0; i < int(header.NumberOfQuestions); i++ {
		questions[i] = *parseQuestion(reader)
	}
	answers := parseNextNSegmentsAsRecords(reader, header.NumberOfAnswers)
	authorities := parseNextNSegmentsAsRecords(reader, header.NumberOfAuthorities)
	additionals := parseNextNSegmentsAsRecords(reader, header.NumberOfAdditionals)
	return &Packet{
		Header:      *parseHeader(reader),
		Questions:   questions,
		Answers:     answers,
		Authorities: authorities,
		Additionals: additionals,
	}
}

func parseIP(data []byte) string {
	var ip strings.Builder
	for _, b := range data[0:3] {
		ip.WriteString(strconv.Itoa(int(b)))
		ip.WriteString(".")
	}
	ip.WriteString(strconv.Itoa(int(data[3])))
	return ip.String()
}
