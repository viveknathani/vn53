package core

import (
	"log"
	"os"
	"testing"
)

func TestLookup(t *testing.T) {
	if Lookup("vivekn.dev", os.Getenv("DEBUG") == "true") != "45.79.126.128" {
		log.Fatal("failed!")
	}
}
