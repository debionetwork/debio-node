#!/usr/bin/env bash

set -e

echo "Creating network"
docker network create --driver=bridge --subnet=172.26.0.0/16 debio
