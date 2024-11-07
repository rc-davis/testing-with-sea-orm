# Testing with SeaORM

How SeaORM can help you build an app that uses Postgres as the main database
and an in-memory SQLite database for integration tests.

## Integration tests

As usual, integration tests can be found in the `tests` directory.

The main benefit of this setup is that it avoids the need for network or file system access in integration tests.
(Note that a full testing strategy would still include a smaller number of end-to-end tests that run on a live Postgres
database.)

The main disadvantage of this setup is that SeaORM, like all ORMs, is a leaky abstraction.
Sometimes you will need to write both Postgres and SQLite specific code. This setup allows you to
switch between the two both in repositories and migrations with the `db.get_database_backend()`
function. This returns a `DbBackend` enum (with `Postgres` and `Sqlite` variants)
which is a required argument for functions that use raw SQL statements, like
`Statement::from_string()` or `Statement::from_sql_and_values()`.

### Sharing Tokio Runtimes in Integration Tests

`[tokio::test]` sets up a different tokio runtime for each test.
This crate annotates integration tests with `#[tokio_shared_rt::test(shared)]`,
because it sets up a single tokio runtime that is shared between integration tests in
the same file. This allows us to set up a single, shared database connection
and run database migrations once before tests begin.

### Debugging and Logging

You can pass some environment variables to the integration test runner for special behavior.

- `DEBUG=true` - This creates a database file for each test rather than using an in-memory
  database, which is slower but easy to inspect. The file has the same name as the
  top-level rust file (with a `.sqlite` extension) and is cleared out at the beginning
  of every test run. This also turns on debug logging.
- `VERBOSE=true` - This turns on debug logging while still using the fast, in-memory database.

### Use with a monorepo

For this setup to work in a monorepo, you would need to do two things:

1. Modify the migrator to run all migrations for all microservices and store migrtion history in a different table.
2. Ensure that every table name is unique across all microservices.

Neither of these things is too difficult for a new system,
but the unique table names requirement will be harder for
systems that have already been deployed.

## SeaORM workflows

### Schema first

This is the recommended approach:

1. Write migrations in `migration/src` directory.
2. Use `sea-orm-cli` to generate entities from the live database:
   `DATABASE_URL="postgres://postgres:postgres@localhost:5432/bakeries_db" sea-orm-cli generate entity -o src/repository/entities`
3. Regenerate entities whenever the database schema changes.

### Entity first

Use methods like [
`create_table_from_entity`](https://docs.rs/sea-orm/*/sea_orm/schema/struct.Schema.html#method.create_table_from_entity)
to bootstrap your database with several handwritten entity files. This is not recommended.

## Other Notes

- The `sea-orm` crate has a [`mock` feature](https://www.sea-ql.org/SeaORM/docs/write-test/mock/), but if you use this
  then you cannot clone a `DatabaseConnection`
  for use in a repository.