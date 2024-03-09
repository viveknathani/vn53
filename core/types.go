package core

// Header describes a DNS header
type Header struct {
	id                  uint16
	flags               uint16
	numberOfQuestions   uint16
	numberOfAnswers     uint16
	numberOfAuthorities uint16
	numberOfAdditionals uint16
}

// Question describes a DNS question
type Question struct {
	name       string
	recordType uint16
	classType  uint16
}

// Record describes a DNS record received in the response
type Record struct{}

// Packet is what gets sent across the network in a DNS query's response
type Packet struct{}

func newHeader(id uint16, flags uint16, numberOfQuestions uint16) *Header {
	return &Header{
		id:                  id,
		flags:               flags,
		numberOfQuestions:   numberOfQuestions,
		numberOfAnswers:     0,
		numberOfAuthorities: 0,
		numberOfAdditionals: 0,
	}
}

func newQuestion(name string, recordType uint16, classType uint16) *Question {
	return &Question{
		name,
		recordType,
		classType,
	}
}
