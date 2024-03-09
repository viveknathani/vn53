build:
	go build -o ./bin/ main.go

test:
	go test -v ./...
	