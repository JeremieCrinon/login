import { requireRole } from "~/lib/auth";
import { AppSidebar } from "~/components/sidebar";

import { EditAccountEmailCard } from "./accountEmailCard";
import { EditAccountPasswordCard } from "./accountPasswordCard";

export function clientLoader() {
  return requireRole("user");
}

export default function Account() {
  return (
    <AppSidebar>
      <div className="flex flex-col gap-4 max-w-md mx-auto mt-10 w-full">
        <EditAccountEmailCard />
        <EditAccountPasswordCard />
      </div>
    </AppSidebar>
  )
}
