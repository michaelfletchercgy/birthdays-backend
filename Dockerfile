# TODO remove all nonpinned versions

#
# Builder Base
########################################

FROM library/debian:testing AS builder

# Add the HTTPS transport
RUN apt-get update
RUN apt-get install -y ca-certificates apt-transport-https gnupg curl

# Add the node key
#RUN apt-key adv --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys 58118E89F3A912897C070ADBF76221572C52609D

# Add node
RUN curl -sL https://deb.nodesource.com/setup_8.x a | bash
RUN apt-get update

# Install all the things.
RUN apt-get install -y nodejs git build-essential pkg-config libpq-dev libssl-dev unzip

# RUST COMPILER
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2018-06-05
ENV PATH=/root/.cargo/bin:$PATH

# Copy the minimal amount in to cache the build resources.
WORKDIR /build
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN touch src/lib.rs
RUN cargo +nightly-2018-06-05 build --release

# Now Compile
COPY . .
RUN cargo +nightly-2018-06-05 build --release


# Runtime
#########
FROM library/debian:testing
LABEL author="Michael Fletcher <m.fletcher@theplanet.ca>"
USER root

RUN apt-get update
RUN apt-get install -y ca-certificates libssl1.1 libpq5

COPY --from=builder /build/target/release/birthdays-backend /opt/birthdays-backend

#ADD target/release/birthdays-backend /opt/birthdays-backend
RUN chmod a+rwx /opt/birthdays-backend
EXPOSE 8111
ENTRYPOINT ["/opt/birthdays-backend"]