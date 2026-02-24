BINARY_NAME=flatted

build:
	go build -o $(BINARY_NAME) main.go

test:
	go clean -testcache
	go test -v -race ./...

lint:
	golangci-lint run ./...
	gosec ./...
	govulncheck ./...
	gofmt -l .

check: test lint

clean:
	rm -f $(BINARY_NAME)

.PHONY: build test clean