FROM golang:alpine
WORKDIR /opt

COPY . .
EXPOSE 8080

RUN go build -o build/hackz-tyranno
ENTRYPOINT ["./build/hackz-tyranno"]