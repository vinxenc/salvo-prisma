```shell
# lint
cargo clippy --fix

# prisma format
cargo prisma

# generate prisma client
cargo prisma generate --schema=./prisma/schema.prisma

# run migration dev
prisma migrate dev

# run migration production
prisma migrate deploy

# run server local
cargo run -p server

# test your local server
curl http://0.0.0.0:5800
```