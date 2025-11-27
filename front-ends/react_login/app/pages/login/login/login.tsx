import { LoginForm } from "./login-form"
import { redirect } from "react-router";

export function clientLoader() {
  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {
    return redirect("/");
  }
  return null;
}

export default function Login() {
  return (
    <LoginForm />
  )
}
