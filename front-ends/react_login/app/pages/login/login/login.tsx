import { LoginForm } from "./login-form"
import { redirect } from "react-router";

export default function Login() {
  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {
    return redirect("/");
  }

  return (
    <div className="gradient-bg h-screen">
      <LoginForm />
    </div>
  )

}
