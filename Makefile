VERSION := $(shell git describe --always --dirty --tags)

.PHONY: default get test fmt all static linux_86 linux_64 linux_arm linux_arm64

default: get
	CGO_ENABLED=0 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o ${GOPATH}/bin/kv

get:
	go get

fmt:
	go fmt ./...

test:
	bash tests/basic.sh

linux_86:
	CGO_ENABLED=0 GOOS=linux GOARCH=386 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-linux_386

linux_64:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-linux_amd64

linux_arm:
	CGO_ENABLED=0 GOOS=linux GOARCH=arm go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-linux_arm

linux_arm64:
	CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-linux_arm64

darwin_64:
	CGO_ENABLED=0 GOOS=darwin GOARCH=amd64 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-darwin_amd64

windows:
	CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -i -ldflags="-X 'main.Version=${VERSION}'" -o build/kv-windows_amd64



