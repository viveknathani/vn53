package core

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"log"
	"math/rand"
	"net"
	"strings"
)

// Lookup returns the IP address for the given hostname
func Lookup(hostname string) []byte {
	const ip = "8.8.8.8"
	conn, err := net.Dial("udp", fmt.Sprintf("%s:53", ip))
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	_, err = conn.Write(buildQuery(hostname))
	if err != nil {
		panic(err)
	}

	response := make([]byte, MAX_DNS_PACKET_SIZE_IN_BYTES)
	_, err = conn.Read(response)
	if err != nil {
		panic(err)
	}

	return response
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
	result.Write(uint16ToBytes(header.id))
	result.Write(uint16ToBytes(header.flags))
	result.Write(uint16ToBytes(header.numberOfQuestions))
	result.Write(uint16ToBytes(header.numberOfAnswers))
	result.Write(uint16ToBytes(header.numberOfAuthorities))
	result.Write(uint16ToBytes(header.numberOfAdditionals))
	return result.Bytes()
}

func questionToBytes(question Question) []byte {
	var result bytes.Buffer
	result.Write(encodeHostname(question.name))
	result.Write(uint16ToBytes(question.classType))
	result.Write(uint16ToBytes(question.recordType))
	return result.Bytes()
}

func buildQuery(hostname string) []byte {
	var query bytes.Buffer
	header := newHeader(uint16(rand.Int()), RECURSION_DESIRED, DEFAULT_NUMBER_OF_QUESTIONS)
	question := newQuestion(hostname, RECORD_TYPE_A, CLASS_TYPE_IN)
	query.Write(headerToBytes(*header))
	query.Write(questionToBytes(*question))
	return query.Bytes()
}
