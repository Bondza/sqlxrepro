### Repro


Setup PostgreSQL with docker or podman
```
# Docker
docker run -d -e POSTGRES_PASSWORD=postgres -p 5433:5432 docker.io/library/postgres:12.6

# or with Podman
podman run -d -e POSTGRES_PASSWORD=postgres -p 5433:5432 docker.io/library/postgres:12.6
```

Apply `schema.sql`

```sh
psql -h localhost -U postgres -p 5433 -f schema.sql
```

Build

```sh
. devenv
cargo build
```