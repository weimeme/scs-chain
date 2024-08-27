# build stage: where we create binary
FROM rust:1.74 AS builder

RUN apt update && apt install -y make clang pkg-config libssl-dev protobuf-compiler build-essential git curl llvm make
RUN rustup default stable && \
    rustup component add rust-src && \
    rustup target add wasm32-unknown-unknown
    # rustup component add rust-src --toolchain 1.74.0-x86_64-unknown-linux-gnu
# rustup default nightly && \
#   rustup update && \
#   rustup update nightly && \
#   rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu\
#   rustup target add wasm32-unknown-unknown

WORKDIR /scs
COPY . /scs
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the scs binary."
FROM docker.io/library/ubuntu:22.04
LABEL description="Multistage Docker image for TSCS Network: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="transxask1@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="SuperEx" 

COPY --from=builder /scs/target/release/scs /usr/local/bin
COPY --from=builder /scs/scripts/validator_node_init.sh /usr/local/bin
COPY --from=builder /scs/scripts/normal_node_init.sh /usr/local/bin

ENV BASE_PATH=/data
ENV SESSION_KEYS_PASSWORD=root
ENV SESSION_KEYS_INDEX=0

RUN useradd -m -u 1000 -U -s /bin/base -d /scs scs && \
	mkdir -p ${BASE_PATH} /scs/.local/share/scs && \
	chown -R scs:scs /data && \
	ln -s ${BASE_PATH} /scs/.local/share/scs && \
# Sanity checks
	ldd /usr/local/bin/scs && \
# # unclutter and minimize the attack surface
# 	rm -rf /usr/bin /usr/sbin && \
    chmod 777 /usr/local/bin/validator_node_init.sh && \
	chmod 777 /usr/local/bin/normal_node_init.sh && \
	/usr/local/bin/scs --version

# RUN /usr/local/bin/scs --version 
USER scs
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/scs", "--chain", "staging", "--database", "auto", "--base-path", "/data" ]
CMD [ "--help" ]
