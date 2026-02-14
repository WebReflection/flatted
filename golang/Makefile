BINARY_NAME=flatted

build:
	go build -o $(BINARY_NAME) main.go

test:
	go test -v ./...

clean:
	rm -f $(BINARY_NAME)

.PHONY: build test clean