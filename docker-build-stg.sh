#!/bin/sh
docker build -t debio_substrate_service .
docker tag debio_substrate_service hub.debio.network/debio_substrate_staging:latest
docker push hub.debio.network/debio_substrate_staging:latest
