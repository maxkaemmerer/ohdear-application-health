#!/bin/bash
cargo clean
cargo build --release --bin ohdear-application-health

TAG="1.4.0"
docker image build . -t "maxkaemmerer/ohdear-application-health:$TAG" -f Dockerfile
docker push "maxkaemmerer/ohdear-application-health:$TAG"