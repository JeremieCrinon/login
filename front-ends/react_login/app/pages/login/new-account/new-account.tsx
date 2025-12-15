import { requireRole } from "~/lib/auth";
import { NewAccountForm } from "./new-account-form";

export function clientLoader() {
  return requireRole("new_account");
}

export default function NewAccount() {
  return (
    <div className="gradient-bg h-screen">
      <NewAccountForm />
    </div>
  )
}
