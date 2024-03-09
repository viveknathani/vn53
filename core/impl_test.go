package core

import (
	"log"
	"testing"
)

func TestLookup(t *testing.T) {
	log.Print(Lookup("www.example.com"))
}
