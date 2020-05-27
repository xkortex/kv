FROM golang:alpine as build

RUN     apk update \
    &&  apk upgrade \
    &&  apk add --no-cache \
            git \
            make \
            bash

WORKDIR $GOPATH/src/github.com/xkortex/kv

COPY . ./

RUN go get

RUN make

FROM build as inline_test

RUN ./tests/basic.sh

FROM scratch

COPY --from=build /go/bin/kv /go/bin/kv

ENTRYPOINT ["/go/bin/kv"]
