# React login
**This is the documentation of the front-end with React of my [login systems suite](https://github.com/JeremieCrinon/login). Please read the [main README.md](../../README.md) first.**

## Getting started
Start by ensuring you have the [requirements](#requirements) and then, follow the [quick start guide](#quick-start-guide). Then edit the files as you like, test, break and explore.

## Requirements
- A working [back-end](../../back-ends).
- [npm](https://https://www.npmjs.com/) or [pnpm](https://pnpm.io)

## Quick start guide
- Clone or download the react_login directory.
- Configure ./app/customConfig.ts.
- Install the package with `npm i` or `pnpm i`.
- Start the app with `npm run dev` or `pnpm run dev`.
- Open http://localhost:5173 in your browser and test everything is working.
- Explore the files, edit and do whatever you want with it.

## Main libraries used
- [React](https://react.dev)
- [React router](https://reactrouter.com/) in framework mode
- [Shadcn/ui](https://ui.shadcn.com/)

## Structure of the app
Most of the code is inside of the app folder.

- ### components folder
This folder contains reusable components that are means to be used in multiple parts of the app, like a backButton or the sidebar. It also contains the ui folder, which contains shadcn/ui components.
- ### pages folder
This folder contains subfolders that contains components that composes the app pages.
- ### types folder
This folder contains some ts interfaces.
- ### translations folder
This folder contains some json files containing internationalization.

## Defining new routes
In a standard react-router app, routes should be defined in the app/routes.ts file. However, for simplifying things, I made the file app/lib/routes.ts that contains all the app routes, and automatically display them in the sidebar if they need to be in. I recommend you adding all your routes in it, even those that are not meant to go to the sidebar, so all routes are in the same file.
