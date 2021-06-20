#!/bin/sh
docker build -t debio_substrate_service .
docker tag debio_substrate_service hub.theapps.dev/debio_substrate_service:latest
docker push hub.theapps.dev/debio_substrate_service:latest

