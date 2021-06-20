#!/bin/sh
echo "clean up old container ........"
docker stop debio_substrate_service || true && docker rm debio_substrate_service || true
docker rmi hub.theapps.dev/debio_substrate_service:latest
docker pull hub.theapps.dev/debio_substrate_service:latest
echo "reload container .............."
docker run -d --name=debio_substrate_service -v /etc/localtime:/etc/localtime:ro -p 0.0.0.0:9944:9944 --restart=always hub.theapps.dev/debio_substrate_service:latest --unsafe-ws-external
