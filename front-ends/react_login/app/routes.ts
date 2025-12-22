import {
  type RouteConfig,
  route
} from "@react-router/dev/routes";

export default [
  route("/", "./pages/origin/origin.ts"),
  route("/login", "./pages/login/login/login.tsx"),
  route("/logout", "./pages/login/logout/logout.ts"),
  route("/new-account", "./pages/login/new-account/new-account.tsx"),
  route("/verify-email", "./pages/login/verify-email/verify-email.tsx"),
  route("/home", "./pages/welcome/welcome.tsx"),
] satisfies RouteConfig;
