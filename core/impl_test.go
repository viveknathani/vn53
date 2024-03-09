package core

import (
	"log"
	"testing"
)

func TestLookup(t *testing.T) {
	if Lookup("vivekn.dev") != "45.79.126.128" {
		log.Fatal("failed!")
	}
}
