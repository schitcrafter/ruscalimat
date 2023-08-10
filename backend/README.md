# ruscalimat-backend

starts as localhost:3000/

## Dev

For sqlx, you need to install sqlx-cli:

```shell
cargo install sqlx-cli
```

Create database:
```shell
sqlx db setup
```

### If you changed migrations, run:

```shell
sqlx db reset
```
If you're changing migrations, you need to run `cargo build` afterwards in order for environments without a running database (i.e. GitHub Actions, other CI/CD) to function. This will change `sqlx-data.json` and/or anything inside `.sqlx`.

