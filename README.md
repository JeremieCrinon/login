# Login templates
### This is the main documentation that provides common information for all the back-ends and front-ends. For more details about a specific front-end or back-end, please refer to the README.md in it's folder.

These templates are made to provide a base for apps requiring a login system. It is intended to choose one back-end, and one or multiple front-end, based on your needs.
All back-ends are HTTP APIs that all have the same routes and return the same results (except in places where it will not affect the front-ends if it is different. ie: an error message destined to developpers).
I tried to the best to my abilities to follow best-practices and make the code as readable as I can.
It also can be a way for you to learn a new technologies, to serve as an example.
It is intended to use a Postgres database with it, you can try to adapt it to other DB, it should not be to hard, but I cannot garanty anything.

**Please not that, for now, it's work in progress, some README.md may be missing, and the code may not be commented as much as I will like it.**


## How can I use it ?
Choose a back-end in the back-ends folder, they have their framework's name in their names, and choose a front-end from the front-ends folder, you can then read the README.md in their folder, start and test them following the instructions in the readme, then start editing them.

## Why using it ?
If your app needs a login system, you won't have to do one from scratch, just edit this one to fit your needs, it also can be a way to learn a new framework, by seeing an example.

## What back-end and front-end should I choose.
This choice is obviously based on your preferences, for now, there is just one back-end and one front-end, so the choice is fairly easy to do, but more framworks in other languages will be added later, and I highly recomend you not to use the template for now if the framworks or languages do not fit your needs, especially with axum in the back-end, which is a Rust framework, it as a really high learning curve and I do not recommend it to beginers.

Here is a list of the frameworks with their pros and cons to help you choose :

### Front-ends :
- Vue.js (JavaScript) : A pretty popular javascript framwork, not really hard to learn for begginer, the code is a bit cleaner than react in my opinion, but it also is less popular.
- More will be comming

### Back-ends : 
- Axum (Rust) : INSANE performances, steep learning curve, not recomended to beginers.
- More will be comming


## How to use it ?
A user with the permission to create users (users with the role `admin` or `edit_users`) can create a user, they will provide the user's email, the role they want to give to the user and the language of the new user (to send an email in their language, internationalization is made in english and french in the emails, you can remove it or add/remove a language if you want), the app will then atribute the new user a random password, and send them an email with the email and password inviting them to connect to their account.
When a new user log in for the first time, they will be greeted with a form to edit their email if they want to and they will have to change their password. After that, they will have to verify their email adress by giving a code that has been sent to their email adress. They will then gain access to the app.
If they have permission to, they will be able to create, edit and delete users, else, they won't be able to do anything as this is just a template, you will have to add the features you want.
There also is a forgot password form in the log in page, and a page to edit the email or password for already loged in users.


## General functionning of the apps

### Authentication
The back-end route `/login` verifies the email and password that have been sent by the front-end, if it is valid, it sends back a JWT containing the id of the user. Then, when a back-end route requiring to be logged in is called, a middleware (or something else depending on the back-end), gets the role required for the route, gets the JWT sent in the request's header, verifies it is valid and gets the user corresponding, and then, verifies the user has the right role (you can also set a route to require to be logged in, but not have a specific role), and then the request is handled if everything went right.
The users are obviously stored in database with their password hashed.
There is a back-end route `/user-infos` that gets the user's JWT token, and tells if it is still valid and what roles the user has. It is used by the front-end when a user logs in to know the user's roles, and when you go back to the app, to verify the token it has stored is still valid.

### Roles
One or multiple roles can be assigned to a user. Here is a list of each role and what they do : 
- admin: Gives all permission to the user.
- user: It can be given to the user depending on the back-end. It gives no permissions at all.
- new_account: Tells that it is a new_account, the user does not have the permission to do anything except call the `/modify-new-account` route.
- unverified_email: Tells that the email hasn't be verified, the user does not have the permission to do anything except call the `verify-email` route and the `/edit-email` route.
- edit_users: Grants the permission to create, read, update and delete other users.

### Back-end routes
Here is a list of the routes with what HTTP method they take, what body and header they expect, and what response they send (not including server errors and missing fields errors).

- `/login` POST route. It expects a JSON body containing an email and password fields. It verifies it is valid, and sends back either a 400 error code to tell either the email or password is invalid (without telling which one, for security reasons), or a 200 code with a JSON containing the token
- `/user-infos` GET route. It expects a Header field `Authorization` with the valud `Bearer token` where token is the token returned by the `/login` route. It responds either with a 401 code if the token isn't valid, or with a 200 code and a Json containing the roles as an array, the user email, and a result which is true if the user is admin, and false if they are not.
- `/modify-new-account`POST route. It excpects a header field with the token, a json body with a new_email and new_password field, and that the user has the roles `new_account`.
...

## Planed framworks to add
### Back-ends
- Symfony (Php)
- Laravel (Php)
- Vapor (Swift)

### Front-ends
- React (JavaScript)
- SwiftUI (Swift) - For IOS, IpadOS, MacOS and WatchOS
- An android version (don't know which language or framwork I will use)
