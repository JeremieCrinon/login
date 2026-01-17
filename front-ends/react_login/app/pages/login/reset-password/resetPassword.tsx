import { useParams, redirect } from "react-router";
import { ResetPasswordForm } from "./resetPasswordForm";


export function clientLoader() {
  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {
      return redirect("/");
  }
}

export default function ResetPassword() {
  const params = useParams();

  const code = params.code!;

  return(
    <div className="gradient-bg w-screen h-screen">
      <ResetPasswordForm code={code} />
    </div>
  )
}
