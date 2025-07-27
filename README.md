# Login templates
**This is the main documentation that provides common information for all the back-ends and front-ends. For more details about a specific front-end or back-end, please refer to the README.md in its folder.**

These templates are made to provide a base for apps requiring a login system. It is intended to choose one back-end, and one or multiple front-ends, based on your needs.
All back-ends are HTTP APIs that all have the same routes and return the same results (except in places where it will not affect the front-ends if it is different. i.e.: an error message destined to developers).
It is intended to follow best practices and make the code as readable as possible.
It also can be a way for you to learn a new technology, to serve as an example.
It is intended to use a PostgreSQL database with it, you can try to adapt it to other DB, it should not be too hard, but I cannot guarantee anything.

**Please note that, for now, it’s a work in progress, some README.md may be missing, and the code may not be commented as much as I would like it.**


## How can I use it ?
Choose a back-end in the back-ends folder, they have their framework's name in their names, and choose a front-end from the front-ends folder, you can then read the README.md in their folder, start and test them following the instructions in the readme, then start editing them.

## Why use it ?
If your app needs a login system, you won't have to do one from scratch, just edit this one to fit your needs, it also can be a way to learn a new framework, by seeing an example.

## What back-end and front-end should I choose?
This choice is obviously based on your preferences. For now, there is just one back-end, so the choice for the back-end is fairly easy to do. But more frameworks in other languages will be added later, and I highly recommend you not to use the template for now if the frameworks or languages do not fit your needs, especially with Axum in the back-end, which is a Rust framework. It has a really high learning curve and I do not recommend it to beginners.

Here is a list of the frameworks with their pros and cons to help you choose :

### Front-ends :
- Vue.js (JavaScript) : A pretty popular JavaScript framework, not really hard to learn for beginners. The code is a bit cleaner than React in my opinion, but it also is less popular.
- SwiftUI (IOS) : The intended way by Apple to create apps for it's platforms. It is still work in progress, documentation, code commenting, small features and support for other platforms are still missing.

### Back-ends : 
- Axum (Rust) : Really good performances, steep learning curve, not recommended to beginners.
- More will be coming


## How to use it ?
A user with the permission to create users (users with the role `admin` or `edit_users`) can create a user. They will provide the user's email, the role they want to give to the user, and the language of the new user (to send an email in their language, internationalization is made in English and French in the emails. You can remove it or add/remove a language if you want). The app will then attribute the new user a random password and send them an email with the email and password inviting them to connect to their account.
When a new user logs in for the first time, they will be greeted with a form to edit their email if they want to, and they will have to change their password. After that, they will have to verify their email address by giving a code that has been sent to their email address. They will then gain access to the app.
If they have permission to, they will be able to create, edit, and delete users. Else, they won't be able to do anything as this is just a template. You will have to add the features you want.
There also is a forgot password form in the login page, and a page to edit the email or password for already logged-in users.


## General functioning of the apps

### Authentication
The back-end route `/login` verifies the email and password that have been sent by the front-end. If it is valid, it sends back a JWT containing the id of the user. Then, when a back-end route requiring being logged in is called, a middleware (or something else depending on the back-end) gets the role required for the route, gets the JWT sent in the request's header, verifies it is valid, and gets the user corresponding. Then, it verifies the user has the right role (you can also set a route to require being logged in, but not have a specific role), and then the request is handled if everything went right.
The users are obviously stored in the database with their password hashed.
There is a back-end route `/user-infos` that gets the user's JWT token and tells if it is still valid and what roles the user has. It is used by the front-end when a user logs in to know the user's roles, and when you go back to the app, to verify the token it has stored is still valid.

### Roles
One or multiple roles can be assigned to a user. Here is a list of each role and what they do : 
- admin: Gives all permissions to the user.
- user: It may be given to the user depending on the back-end. It gives no permissions at all.
- new_account: Tells that it is a new account, the user does not have the permission to do anything except call the `/modify-new-account` route.
- unverified_email: Tells that the email hasn't been verified, the user does not have the permission to do anything except call the `verify-email` route and the `/edit-email` route.
- edit_users: Grants the permission to create, read, update, and delete other users.

### Back-end routes
Here is a list of the routes with what HTTP method they take, what body and header they expect, and what response they send (not including server errors and missing fields errors).

#### Login routes
These routes are intended to be used by any user, for login purposes.
- `/login` POST route. It expects a JSON body containing an email and password fields. It verifies it is valid, and sends back either a 400 error code to tell either the email or password is invalid (without telling which one, for security reasons), or a 200 code with a JSON containing the token.
- `/user-infos` GET route. It expects a Header field `Authorization` with the value `Bearer token` where token is the token returned by the `/login` route. It responds either with a 401 code if the token isn't valid, or with a 200 code and a JSON containing the roles as an array, the user email, and a result which is true if the user is admin, and false if they are not.
- `/modify-new-account`POST route. It expects a header field with the token, a JSON body with a new_email and new_password field, and that the user has the roles `new_account`. It verifies that the email and password have a correct shape (the email is a valid email, and password has one uppercase letter, one lowercase letter, a number, a special char, and is at least 8 characters long), verifies that the new email isn't used by another user (except the user that called the route themselves, they might want to keep the same email). If everything is right, it edits the user in BD, puts them the `unverified_email` role if they don't already have it and removes the `new_account` role, puts a randomly generated email verification code for the user in DB and sends an email with the email verification code.
- `/verify-email` POST route. It expects the header field with the token, a JSON body with the email verification code that has been sent by email to them. It then verifies that the verification code is the right for the user, and returns a 400 code if the code isn't the right one if the code is the right one, it removes the code for the user in DB, removes the role `unverified_email` and returns a 200 code.
- `/edit-email` POST route. It expects the header field with the token, a user that does not have the `new_account` role (but the user can have the `unverified_email` role), and a JSON body with a new_email field and a password field that contains the user's password (the password field isn't required if the user has the `unverified_email` role, to simplify things). This route is intended to be used to either send a new email verification code if the user hasn't received the mail (by calling the route with the same email in the body), edit the email if the user hasn't verified their email and made an error in it, or to edit the email when the user is logged in. It will verify that the password sent is right (if the user does not have the `unverified_email role`), that another user does not have the new_email. It will then create a random email verification code, put it for the user in DB, give them the role `unverified_email`, send an email with the verification code, and return a 200 code.
- `/edit-password` POST route. It expects the header field with the token, a JSON body with a current_password field, and a new_password field. It verifies that the new_password respects the conditions for a password, verifies that the current_password is correct, and then hashes the new_password and edits the user in DB with it. It then returns a 200 code.
- `/forgot-password` POST route. It expects a JSON body with the user's email. If it finds a user with this email, it will send an email with a link to reset the password (with a randomly generated code in that link), put that randomly generated code for the user in DB, and return a 200 code. It will also return the same response if the email does not exist, for security reasons.
- `/reset-password` POST route. It expects a JSON body with the password resetting code and the new_password. The front-end will take care of getting the code in the link to send it to the back-end. It will then search a user with the code. If there is one, it will then edit the password with the new one that it has hashed and return a 200 code; if it does not find a user, it means the code isn't valid, and it will then return a 401 response.

#### Users routes
These routes are intended to be used by users with the role `admin` or `edit_users` to create, read, update, or delete users.
- `/users` GET route. It expects the header field with the token. It returns a list of all the users with limited information about the user.
- `/users/id` GET route. It expects the header field with the token. It returns a 404 error if the id in the route does not correspond to any user, or it returns a 200 code with the details of the user.
- `/users` POST route. It expects the header field with the token and a JSON body with the email of the new user and an array of the user's role (that can be empty if we don't want to give the new user any role). It will then verify that the new user's email isn't already taken and then create a user with the email and roles provided in the request, plus the role `new_account` and a hashed password that has been randomly generated. It will then send an invite email to the new user with the randomly generated password. It will then return a 200 code.
- `/users/id` DELETE route. It expects the header field with the token. It returns a 404 error if the id in the route does not correspond to any user or deletes the user in DB and returns a 200 code.
- `/users/id/roles` PUT route. It expects the header field with the token and a JSON body with the roles field. It returns a 404 error if the id in the route does not correspond to any user; else, it will edit the user in DB with the new roles and return a 200 code.
- `/users/id/email` PUT route. It expects the header field with the token and a JSON body with the email field. It returns a 404 error if the id in the route does not correspond to any user; else, it will verify that the email isn't already taken, then generate a random email verification code, send an email with this verification code, and edit the user in DB with this new email, email verification code, and add the role `unverified_email`. It will then return a 200 code.
- `/users/list-roles` GET route. It expects the header fields with the token. It returns the list of all roles that can be given to users.

## Planned frameworks to add
### Back-ends
- Symfony (PHP)
- Laravel (PHP)
- Vapor (Swift)

### Front-ends
- React (JavaScript)
- An Android version (don't know which language or framework I will use)

## Contributing
I am all open to contributions, but I will not accept PRs in other frameworks/languages than the one already in the project or in the [Planned frameworks to add section](#planned-frameworks-to-add).
