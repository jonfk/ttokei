docker run -it --rm --network=ttokeidocker_postgres_net --link ttokeidocker_postgresdb_1:postgres postgres psql -h ttokeidocker_postgresdb_1 -U postgres
