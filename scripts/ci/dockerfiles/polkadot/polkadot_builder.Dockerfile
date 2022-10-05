# This is the build stage for Cherry. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /cherry
COPY . /cherry

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Cherry binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Cherry: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="liompis@cherrylabs.org" \
	io.parity.image.vendor="Cherry Network" \
	io.parity.image.description="Cherry implementation" \
	io.parity.image.source="https://github.com/CherryNetwork/cherry/blob/${VCS_REF}/scripts/ci/dockerfiles/cherry/cherry_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/CherryNetwork/cherry/"

COPY --from=builder /cherry/target/release/cherry /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /cherry cherry && \
	mkdir -p /data /cherry/.local/share && \
	chown -R cherry:cherry /data && \
	ln -s /data /cherry/.local/share/cherry && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/cherry --version

USER cherry

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/cherry"]
