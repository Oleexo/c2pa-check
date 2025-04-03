FROM golang:1.24 AS builder
ARG APPUSER=appuser
ARG TARGETOS
ARG TARGETARCH
ENV USER=${APPUSER}
ENV UID=1001
WORKDIR /source
COPY [ "go.mod", "go.mod" ]
COPY [ "go.sum", "go.sum" ]
RUN go mod download
COPY [ "cmd/", "cmd/" ]
COPY [ "internal/", "internal/" ]
RUN CGO_ENABLED=0 GOOS=linux GOOS=${TARGETOS} GOARCH=${TARGETARCH} go build -o ./app/main ./cmd/api/

FROM ubuntu:latest AS c2patool
WORKDIR /scripts
RUN apt-get update && apt-get install -y curl wget 
COPY [ "scripts/download.sh", "download.sh"]
RUN chmod +x ./download.sh && ./download.sh


FROM ubuntu:latest
#ARG APPUSER=appuser
ENV C2PA_TOOL_PATH=/bin/c2patool
#ENV HOME /home/$APPUSER
#RUN adduser -D "${APPUSER}"
RUN apt-get update && apt-get install -y libc6

COPY --from=c2patool --chown=${APPUSER}:${APPUSER} /scripts/c2patool/c2patool /bin/c2patool
COPY --from=builder --chown=${APPUSER}:${APPUSER} /source/app /app
WORKDIR /app
#USER ${APPUSER}

EXPOSE 8080

ENTRYPOINT ["./main"]