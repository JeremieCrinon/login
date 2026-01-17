import { redirect } from "react-router";

export function requireRole(role?: string) {
  const userToken: string = sessionStorage.getItem("token") ?? "";

  if (userToken == "") {
    return redirect("/"); // If there is not userToken, it means the user isn't logged in.
  }

  const userRoles: string[] = JSON.parse(sessionStorage.getItem("user_roles") || "[]"); // Get the roles from the sessionStorage

  if (userRoles.includes("new_account") && role !== "new_account") return redirect("/new-account"); // If the user has the new_account role and doesn't go to a route for new accounts, we redirect them to the new-account route
  if (userRoles.includes("unverified_email") && (role !== "unverified_email" && role !== "new_account")) return redirect("/verify-email"); // Same as above but for unverfied_email, we also verify the required role (role) isn't new-account as user may have the unverified_email and the new_account role

  if (role) {
    const isAdmin = userRoles.includes("admin");
    const hasRole = userRoles.includes(role) || role === "user";

    if (!(hasRole || isAdmin)) return redirect("/"); // If the user tries to get to a route they don't have the role for, they get redirected to the origin
  }

  return null;
}
