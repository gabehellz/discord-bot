# Discord Bot

Minimal Discord Bot for one server only.

## How To Use

Clone this repo, copy `.env.example` to `.env` and setup environment variables. For the `DATABASE_URL` one, you must follow this pattern: `sqlite://data.db`, where `data.db` is the filename of the database.

Then, you should be able to run this project:

``` shell
cargo run
```

Note: migrations are run when you execute the code.
