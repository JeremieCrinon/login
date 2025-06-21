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

## Commands
- Refresh DB via sea-orm (reset DB and execute migrations)
`sea-orm-cli migrate refresh`
- Execute sea-orm new migrations (Up)
*Please not that this is automatically done at app's startup, and it should not need to be done via CLI*
`sea-orm-cli migrate up`
- Run the app in debug mode
`cargo run`
- Run the app in release mode
`cargo run --release`
- Build the app in debug mode
`cargo build`
- Build the app in release mode
`cargo build --release`


## Main libraries / frameworks used
Here is a list of the most important libraries and frameworks used :
- Axum for the HTTP and routes handling
- Sea-orm for the database connection and ORM
- Dotenv for getting the env values
- Lettre for sending emails
- Tera for the email templates

## Strucure of the app
The migration folder contains the sea-orm migrations. The src folder containes the the rust code of the app, the templates folder contains the tera templates, the translations folder containes the translations in the form of one json file per language, the target folder contains the compiled app.

- ### Main code folder (src folder)
WIP

- ### Migrations folder
You can find the migrations in the src folder, for more details, you can go to the [sea-orm documentation](https://www.sea-ql.org/SeaORM/docs/index), it is just a standard migrations folder for sea-orm.   
*Please note that the sea-orm initialisation isn't in this folder, you can find more details about it in the [Sea-orm](#sea-orm) part of the doc.*

- ### Templates folder
All the email templates are found here. The `base_email.html.tera` file contains the base structure of the mails, the `style.css` file is the css file for all the templates. Then, there is a folder per email, in these folder, you can fin an `email.html.tera` file, and an `email.txt.tera` file, the `email.html.tera` extends the `base_email.html.tera`, it is the HTML template, the `email.txt.tera` just contains the plain text backup.   
*Please not, that, if you have never done any email templates, the HTML and CSS are really limited, and you cannot do the same things as in the browser.*

- ### Translations folder
You can find one json file per language in this folder, if you only want english or another language, just delete the languages json file you do not want, and refer to the [translations](#translations) part of the doc.

- ### Target Folder
You can find all the compiled apps here, please refer to the [rust documentation](https://doc.rust-lang.org/book) for more details.

## Sea-orm
WIP

## Translations
WIP