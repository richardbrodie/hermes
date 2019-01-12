#!/bin/bash

until nc -z $DB_HOST 5432;
do
    echo "not ready"
    sleep 1
done

sleep 1
echo "Postgres is up - executing migrations"
/usr/bin/diesel migration run --database-url="postgres://${PG_USER}:${PG_PASS}@${DB_HOST}/${PG_DB}"

./hermes
