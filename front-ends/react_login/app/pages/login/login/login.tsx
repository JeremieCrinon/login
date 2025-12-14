import { LoginForm } from "./login-form"
import { redirect } from "react-router";

export function clientLoader() {
  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {
    return redirect("/");
  }
}

export default function Login() {

  return (
    <div className="gradient-bg h-screen">
      <LoginForm />
    </div>
  )

}
