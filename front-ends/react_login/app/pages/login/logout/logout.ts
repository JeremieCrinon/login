import { redirect } from "react-router";

export function clientLoader() {
  sessionStorage.removeItem("token");
  localStorage.removeItem("token");
  return redirect("/");
}

export default function Logout() {
  return null;
}
