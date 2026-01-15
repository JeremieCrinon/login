import { requireRole } from "~/lib/auth";
import { AppSidebar } from "~/components/sidebar";

import { EditAccountEmailCard } from "./accountEmailCard";

export function clientLoader() {
  return requireRole("user");
}

export default function Account() {
  return (
    <AppSidebar>
      <EditAccountEmailCard />
    </AppSidebar>
  )
}
