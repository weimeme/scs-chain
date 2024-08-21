# This is the build stage for scs. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /scs
COPY . /scs
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the scs-node binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Multistage Docker image for TSCS Network: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="transxask1@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="SuperEx" 

COPY --from=builder /scs/target/release/scs-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /scs scs && \
	mkdir -p /data /scs/.local/share/scs && \
	chown -R scs:scs /data && \
	ln -s /data /scs/.local/share/scs && \
# Sanity checks
	ldd /usr/local/bin/scs && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
	/usr/local/bin/scs-node --version 

USER scs
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]