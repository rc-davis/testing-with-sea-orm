# SeaOrm Migration Example

## Things to Note

- You have to create the database before you can run migrations.
- Every migration must be added to `lib.rs`.
    - `main.rs` can be used to run all migrations, but is not actually part of the library.
- Every table `foo` has an enum `Foo` used in migration code.
    - Variants
        - A special variant `Foo::Table` for the table itself
        - A varient for each column (e.g. `Foo::UserId` maps to `user_id`)
    - These enums are conventionally defined in the migration file that creates or modifies a table, but they could be
      placed elsewhere.
- The SeaORM migrator executes Postgres migrations in a transaction. That means you can't create indexes concurrently if
  you use it.
    - You could, however, write your own migrator that doesn't do this.

## Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```
