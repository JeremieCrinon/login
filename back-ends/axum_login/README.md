# Axum login
**This is the documentation of the back-end with Axum of my [login systems suite](https://github.com/JeremieCrinon/login).**

## How to start ?
I recommend you to start by verifying you have the [requirements](#requirements), and then, to follow the [quick start guide](#quick-start-guide). Then, you will be able to test the app with an HTTP client, like [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest). You will then be able to connect a front-end to it, and edit them.

## Requirements
- Latest version of Rust, you can install it via [rustup](https://rustup.rs) if you haven't already.
- Sea-orm-cli, you can install it via cargo: `cargo install sea-orm-cli`.
- PostgreSQL
- Mailhog is recommended to test the email sending without actually sending emails.

## Quick start guide
- Get the axum_login directory either by doing a git clone, or downloading the app trough github, or however you want.
- Create an empty database in Postgres.
- Configure your .env by copying the .env.example, and enter the values you need, there is comments on each line explaining what you should enter.
- Execute the command `cargo run`, it should start without errors.
- Test the routes with [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest), you have the list of the routes and what they expect on the .env of the [full login systems suite](https://github.com/JeremieCrinon/login).