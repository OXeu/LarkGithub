FROM rust:1.81.0 as builder
ENV NAME=lark-github-issue

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src/${NAME}
# Now copy the sources and do the real build
COPY . .
RUN cargo build --release 

# Second stage putting the build result into a debian jessie-slim image
FROM debian
ENV NAME=lark-github-issue
# Update certificate store
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}
CMD /usr/local/bin/${NAME}