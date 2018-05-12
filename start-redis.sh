#! /bin/bash

DATA=`realpath ./data`
mkdir -p "${DATA}"
docker run --name payme-redis-dev -v "${DATA}:/data" --net host -d redis:3.2 redis-server --appendonly yes
