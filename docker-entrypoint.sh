#!/bin/bash

until nc -z $DB_HOST 5432;
do
    echo "not ready"
    sleep 1
done
echo "Postgres is up - executing command"

./hermes
