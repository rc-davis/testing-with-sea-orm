# bakery-backend

An example of SeaORM usage.

## Integration tests

The main benefit of this setup is the ability to run integration tests against an in-memory SQLite database
instead of a live Postgres database, thus avoiding the need for network or file system access in most tests.
A full testing strategy would still include a smaller number of end-to-end tests that run on a live Postgres database.

The main disadvantage of this setup is that SeaORM, like all ORMs, is a leaky abstraction.
Sometimes you will need to write both Postgres and SQLite specific code. You can switch at runtime or compile time.

- **Runtime switching**: The `db.get_database_backend()` function returns an enum with
  `DbBackend::Postgres` and `DbBackend::Sqlite` variants.
  This is a required argument for functions that use raw SQL statements, like
  `Statement::from_string()` or `Statement::from_sql_and_values()`.
- **Compile time switching**: If you always use SQLite for testing and Postgres otherwise, you can use
  `#[cfg(not(test))]` for Postgres and `#[cfg(test)]` for SQLite.

### Use with a monorepos

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