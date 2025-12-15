import { redirect } from "react-router";
import axios from "axios";

import { API_URL } from "~/customConfig";

async function checkToken(): Promise<boolean> {
  const token = sessionStorage.getItem("token") ?? localStorage.getItem("token")!
  let result = false;

  await axios.get(`${API_URL}/user-infos`, {
    headers: {
      "Authorization": `Bearer ${token}`
    }
  })
    .then((response) => {
      sessionStorage.setItem("token", token); // Set the token in the sessionStorage in case it just is in the localStorage
      sessionStorage.setItem("user_email", response.data.user_mail);
      sessionStorage.setItem("user_roles", JSON.stringify(response.data.roles));
      result = true;
    })
    .catch((error) => {
      if (error.status !== 401) {
        console.error(error);
      }

      result = false;
    })

  return result;
}

export async function clientLoader() {


  if (sessionStorage.getItem("token") || localStorage.getItem("token")) {

    const result = await checkToken();

    if (!result) {
      return redirect("/logout");
    }

    let roles = sessionStorage.getItem("user_roles");

    if (roles!.includes("new_account")) {
      return redirect("/new-account");
    } else if (roles!.includes("unverified_email")) {
      return redirect("/verify-email");
    } else {
      return redirect("/home");
    }

  }

  return redirect("/login")
}

export default function Origin() {
  return null;
}
