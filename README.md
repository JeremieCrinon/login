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
- SwiftUI (Swift, IOS) : The intended way by Apple to create apps for it's platforms. It is still work in progress, documentation, code commenting, small features and support for other platforms are still missing.

### Back-ends : 
- Axum (Rust) : Really good performances, steep learning curve, not recommended to beginners.
- Vapor (Swift) : Excellent compromise between PHP frameworks and lower level languages, with a not to steep leaning curve, ease of use and speed of development, but with good performances.
- More will be coming


## How to use it ?
A user with the permission to create users (users with the role `admin` or `edit_users`) can create a user. They will provide the user's email, the role they want to give to the user, and the language of the new user (to send an email in their language, internationalization is made in English and French in the emails. You can remove it or add/remove a language if you want). The app will then attribute the new user a random password and send them an email with the email and password inviting them to connect to their account.
When a new user logs in for the first time, they will be greeted with a form to edit their email if they want to, and they will have to change their password. After that, they will have to verify their email address by giving a code that has been sent to their email address. They will then gain access to the app.
If they have permission to, they will be able to create, edit, and delete users. Else, they won't be able to do anything as this is just a template. You will have to add the features you want.
There also is a forgot password form in the login page, and a page to edit the email or password for already logged-in users.


## General functioning of the apps

### Roles
One or multiple roles can be assigned to a user. Here is a list of each role and what they do : 
- admin: Gives all permissions to the user.
- user: It may be given to the user depending on the back-end. It gives no permissions at all.
- new_account: Tells that it is a new account, the user does not have the permission to do anything except call the `/modify-new-account` route.
- unverified_email: Tells that the email hasn't been verified, the user does not have the permission to do anything except call the `verify-email` route and the `/edit-email` route.
- edit_users: Grants the permission to create, read, update, and delete other users.

### Back-ends
Please refer to the dedicated [README.md](./back-ends/README.md) of the back-ends for more details about them.

### Front-ends
The front-ends are "just" displaying an interface with forms and calling the back-ends. They make as much verifications as possible before calling the back-ends to prevent unecessary requests.

## Planned frameworks to add
### Back-ends
- Symfony (PHP)
- Laravel (PHP)
- NestJS (NodeJS)

### Front-ends
- React (JavaScript)
- An Android version (don't know which language or framework I will use)

## Contributing
I am all open to contributions, but I will not accept PRs in other frameworks/languages than the one already in the project or in the [Planned frameworks to add section](#planned-frameworks-to-add).
