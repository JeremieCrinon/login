import { requireRole } from "~/lib/auth";
import { AppSidebar } from "~/components/sidebar";

export function clientLoader() {
  return requireRole("user");
}

export default function Home() {
  return (
    <AppSidebar>
      <h1>Home page</h1>
      <p>Customize for the app</p>
    </AppSidebar>
  );
}
