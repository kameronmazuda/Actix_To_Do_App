#!/bin/bash

cd ..
docker-compose up -d

until docker run -it postgres --add-host host.docker.internal:host-gateway docker.io/postgres:14-alpine -h localhost -e POSTGRES_PASSWORD=password -U username pg_isready
do
    echo "Waiting for postgres"
    sleep 2;
done

echo "docker is now runnning"
docker-compose down