package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/viveknathani/vn53/core"
)

func main() {
	fmt.Printf("Welcome to vn53! Start by giving in some domain name. Type `quit` when you are done.\n")
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("> ")
		scanner.Scan()
		input := scanner.Text()
		if input == "quit" {
			fmt.Printf("goodbye!\n")
			break
		}
		ip := core.Lookup(input, os.Getenv("DEBUG") == "true")
		fmt.Printf(">> The IP is %s.\n", ip)
	}
}
