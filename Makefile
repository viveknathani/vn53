build:
	go build -o ./bin/ main.go

test:
	export DEBUG=true && go test -v ./...

run:
	export DEBUG=false && ./bin/main

run-debug:
	export DEBUG=true && ./bin/main
	