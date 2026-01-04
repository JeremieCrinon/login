import { requireRole } from "~/lib/auth";
import { VerifyEmailForm } from "./verify-email-form";

export function clientLoader() {
  return requireRole("unverified_email");
}

export default function VerifyEmail() {
  return (
    <div className="gradient-bg w-screen h-screen">
      <VerifyEmailForm />
    </div>
  )
}
