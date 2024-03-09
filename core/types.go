package core

// Header describes a DNS header
type Header struct {
	ID                  uint16
	Flags               uint16
	NumberOfQuestions   uint16
	NumberOfAnswers     uint16
	NumberOfAuthorities uint16
	NumberOfAdditionals uint16
}

// Question describes a DNS question
type Question struct {
	Name       []byte
	RecordType uint16
	ClassType  uint16
}

type internalQuestion struct {
	RecordType uint16
	ClassType  uint16
}

// Record describes a DNS record received in the response
type Record struct {
	Name       []byte
	RecordType uint16
	ClassType  uint16
	TTL        uint32
	Data       []byte
}

type internalRecord struct {
	RecordType uint16
	ClassType  uint16
	TTL        uint32
	DataSize   uint16
}

// Packet is what gets sent across the network in a DNS query's response
type Packet struct {
	Header      Header
	Questions   []Question
	Answers     []Record
	Authorities []Record
	Additionals []Record
}

func newHeader(id uint16, flags uint16, numberOfQuestions uint16) *Header {
	return &Header{
		ID:                  id,
		Flags:               flags,
		NumberOfQuestions:   numberOfQuestions,
		NumberOfAnswers:     0,
		NumberOfAuthorities: 0,
		NumberOfAdditionals: 0,
	}
}

func newQuestion(name []byte, recordType uint16, classType uint16) *Question {
	return &Question{
		Name:       name,
		RecordType: recordType,
		ClassType:  classType,
	}
}
