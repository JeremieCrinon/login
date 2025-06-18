# Login templates
### This is the main documentation that provides common information for all the back-ends and front-ends. For more details about a specific front-end or back-end, please refer to the README.md in it's folder.

These templates are made to provide a base for apps requiring a login system. It is intended to choose one back-end, and one or multiple front-end, based on your needs.
All back-ends are HTTP APIs that all have the same routes and return the same results (except in places where it will not affect the front-ends if it is different. ie: an error message destined to developpers).
I tried to the best to my abilities to follow best-practices and make the code as readable as I can.
It also can be a way for you to learn a new technolies, to serve as an example.
**Please not that, for now, it's work in progress, some README.md may be missing, and the code may not be commented as much as I will like it.**


## How can I use it ?
Choose a back-end in the back-ends folder, they have their framework's name in their names, and choose a front-end from the front-ends folder, you can then readme the README.md in their folder, start and test them, then start editing them.

## Why using it ?
If your app needs a login system, you won't have to do one from scratch, just edit this one to fit your needs, it also can be a way to learn a new framework, by seeing an example.

## What back-end and front-end should I choose.
This choice is obviously based on your preferences, for now, there is just one back-end and one front-end, so the choice is fairly is to do, but more framworks in other languages will be added later, and I highly recomend you not to use the template for now if the framworks or languages do not fit your needs, especially with axum in the back-end, which is a Rust framework, it as a really high learning curve and I do not recommend it to beginers.
Here is a chart with the frameworks, and their pros and cons :

### Front-ends :
- Vue.js : A pretty popular javascript framwork, not really hard to learn for begginer, the code is a bit cleaner than react in my opinion, but it also is less popular.

### Back-ends : 
- Axum : INSANE performances, steep learning curve, not recomended to beginers.


## How to use it ?
A user with the permission to create users (users with the role `admin` or `edit_users`) can create a user, they will provide the user's email, the role they want to give to the user and the language of the new user (to send an email in their language, internationalization is made in english and french in the emails, you can remove it or add/remove a language if you want), the app will then atribute the new user a random password, and send them an email with the email and password inviting them to connect to their account.
When a new user log in for the first time, they will be greeted with a form to edit their email if they want to and they will have to change their password. After that, they will have to verify their email adress by giving a code that has been sent to their email adress. They will then gain access to the app.
If they have permission to, they will be able to create, edit and delete users, else, they won't be able to do anything as this is just a template, you will have to add the features you want.
There also is a forgot password form in the log in page, and a page to edit the email or password for already loged in users.