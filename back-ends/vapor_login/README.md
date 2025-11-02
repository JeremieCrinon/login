# Vapor login
**This is the documentation of the back-end with Vapor of my [login systems suite](https://github.com/JeremieCrinon/login). Please read the [main README.md](../../README.md) and the [back-ends general README.md](../README.md) first.**

## Getting started
Start by ensuring you have the [requirements](#requirements), and then, follow the [quick start guide](#quick-start-guide). Then, you will be able to test the app with an HTTP client, like [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest). You will then be able to connect a front-end to it and edit them.

## Requirements
You can run the app, tests, migrations and build without Docker, I will however use it in this guide as this should work with any OS.
- Docker
- Docker compose
- A running PostgreSQL server
- A [mailgun](https://mailgun.com) account ready to use

## Quick start guide
Even tough Docker is used in this guide, it is only for dev environments, for production environments, please build the app and then create your own Docker configs or host it another way.
- Clone or download the vapor_login directory.
- Create an empty database in Postgres
- Configure your .env by copying the .env.example, and enter the values you need, there are comments on each line explaining what you should enter.
- Create the templates in your mailgun account. Example templates are provided in [mailgun-templates.md](./mailgun-templates.md).
- Execute the command `docker compose up test` it should run the test without errors. (It will display that some routes returned a 4.. error, this is normal as some tests expects clients errors to be returned, what is important is that all the test suites have runned successfully at the end).
- Execute the command `docker compose up migrate`, it should migrate without errors. Verifty then that the database schema as successfully been created.
- Execute the command `docker compose up run`, it should start without errors.
- Test the routes with [Postman](https://www.postman.com) or [Insomnia](https://insomnia.rest), you have the list of the routes and what they expect on the .env of the [back-ends](../README.md).

## Structure of the app
### Sources/App folder
This folder contain other folders containing all the app's code, plus some files :
- #### routes.swift
This file registers the app's controllers. It also contains logic for some really simple routes.
- #### Controllers folder
This folder contains the controllers, that contains all the routes logic
- #### Helpers folder
This folder contains a few helper functions.
- #### Middlewares folder
This folder contains the AuthMiddleware that verifies user's authentication and roles for routes that needs it.

### Tests folder
This folder contains all the unit tests.

## TODO
Add the build command in the docker-compose.yml and check the app in a production environment.
