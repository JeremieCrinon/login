import { requireRole } from "~/lib/auth";
import { NewAccountForm } from "./newAccountForm";

export function clientLoader() {
  return requireRole("new_account");
}

export default function NewAccount() {
  return (
    <div className="gradient-bg w-screen h-screen">
      <NewAccountForm />
    </div>
  )
}
