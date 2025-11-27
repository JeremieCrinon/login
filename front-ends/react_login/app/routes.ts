import {
  type RouteConfig,
  route
} from "@react-router/dev/routes";

export default [
  route("/", "./pages/welcome/welcome.tsx"),
  route("/login", "./pages/login/login/login.tsx"),
] satisfies RouteConfig;
