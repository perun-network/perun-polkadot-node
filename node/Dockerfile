FROM paritytech/ci-linux:production

WORKDIR /var/www/node-template
COPY . .
RUN cargo build

FROM ubuntu:20.04

WORKDIR /bin
EXPOSE 9944
COPY --from=0 /var/www/node-template/target/debug/node-template .
CMD [ "./node-template", "--dev", "--tmp", "--ws-external" ]
