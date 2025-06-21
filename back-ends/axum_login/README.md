# Axum login
**This is the documentation of the back-end with Axum of my [login systems suite](https://github.com/JeremieCrinon/login).**

## Getting started
Start by ensuring you have the [requirements](#requirements), and then, follow the [quick start guide](#quick-start-guide). Then, you will be able to test the app with an HTTP client, like [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest). You will then be able to connect a front-end to it and edit them.

## Requirements
- Latest version of Rust, you can install it via [rustup](https://rustup.rs) if you haven't already.
- Sea-orm-cli, you can install it via cargo: `cargo install sea-orm-cli`.
- A running PostgreSQL server
- Mailhog is recommended to test the email sending without actually sending emails.

## Quick start guide
*Please note that I am on macOS, I have tested it on an empty macOS VM to know every dependency needed. You might need more dependencies on other OS, if you have any problem, please let me know so I can fix the documentation and help you if you need to.*
- Clone or download the axum_login directory.
- Create an empty database in Postgres.
- Configure your .env by copying the .env.example, and enter the values you need, there are comments on each line explaining what you should enter.
- Execute the command `cargo run`, it should start without errors.
- Test the routes with [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest), you have the list of the routes and what they expect on the .env of the [full login systems suite](https://github.com/JeremieCrinon/login).

## Commands
- Refresh DB via sea-orm (reset DB and execute migrations)
`sea-orm-cli migrate refresh`
- Execute sea-orm new migrations (Up)
*Please note that this is automatically done at app's startup, and it should not need to be done via CLI*
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

## Structure of the app
The migration folder contains the sea-orm migrations. The src folder contains the Rust code of the app, the templates folder contains the Tera templates, the translations folder contains the translations in the form of one JSON file per language, the target folder contains the compiled app.

- ### Main code folder (src folder)
This folder contains other folders, plus the main.rs file :
- #### main.rs file
This is the entry file, the first file that is called, it will then call functions and instantiate classes from other files.
- #### routes folder
The mod.rs contains a function to instantiate the Axum router, the other files also instantiate an Axum router with routes that will be merged into the main router.
- #### handlers folder
Contains functions that will be called by the router to do the logic when a route is called.
- #### middleware folder
Contains middleware that are used to put logic between the router and an handler (like for verifying user's authentication).
- #### helpers folder
Contains functions to do logic in other places of the app, used when the logic needs to be done at multiple places or that the code becomes too messy.
- #### db folder
Contains a function to connect sea-orm to the database and execute the migrations up at the app's startup.
- #### entities folder
Contains the entities automatically created by sea-orm (if you do migrations-first, which is recommended for sea-orm) that you can edit to fit exactly what is needed.
- #### translator folder
Contains a class that is instantiated at app's startup. It reads the translations JSON files, parses them, and has a function to return the one in the language wanted.

- ### Migrations folder
You can find the migrations in the src folder. For more details, you can go to the [sea-orm documentation](https://www.sea-ql.org/SeaORM/docs/index). It is just a standard migrations folder for sea-orm.   
*Please note that the sea-orm initialisation isn't in this folder. This is just the migrations. The initialisation is in the [db folder](#db-folder).*

- ### Templates folder
All the email templates are found here. The `base_email.html.tera` file contains the base structure of the mails. The `style.css` file is the CSS file for all the templates. Then, there is a folder per email. In these folders, you can find an `email.html.tera` file and an `email.txt.tera` file. The `email.html.tera` extends the `base_email.html.tera`. It is the HTML template. The `email.txt.tera` just contains the plain text backup.   
*Please note that, if you have never done any email templates, the HTML and CSS are really limited, and you cannot do the same things as in the browser.*

- #### Translations folder
You can find one JSON file per language in this folder. If you only want English or another language, just delete the languages JSON file you do not want and delete/add the reference to the language in the files in the [translator folder](#translator-folder).

- ### Target Folder
You can find all the compiled apps here. Please refer to the [Rust documentation](https://doc.rust-lang.org/book) for more details.
