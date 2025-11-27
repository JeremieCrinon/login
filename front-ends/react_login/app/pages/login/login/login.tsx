import { LoginForm } from "./login-form"
import { redirect } from "react-router";
import { useEffect } from "react";

export function clientLoader() {
  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {
    return redirect("/");
  }
  return null;
}

export default function Login() {

  return (
    <div className="gradient-bg h-screen">
      <LoginForm />
    </div>
  )

}
