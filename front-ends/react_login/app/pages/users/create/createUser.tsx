import { useEffect, useState } from "react";
import { requireRole } from "~/lib/auth";
import { CreateUserForm } from "./createUserForm";
import { AppSidebar } from "~/components/sidebar";
import axios from "axios";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";

import { API_URL } from "~/customConfig";

export function clientLoader() {
  return requireRole("edit_users");
}

export default function createUser() {
  const token = sessionStorage.getItem("token");
  const [roles, setRoles] = useState<String[]>([]);
  const [loading, setLoading] = useState(true);

  const { t } = useTranslation();
  const navigate = useNavigate();

  function getRoles() {
    setLoading(true);
    axios.get(`${API_URL}/users/list-roles`, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
    .then((r) => {
      setRoles(r.data.roles);
      setLoading(false);
      console.log(r.data.roles);
    })
    .catch((e) => {
      console.error(e);

      toast(t("users.create.error.roles.title"), {
        description: t("users.create.error.roles.description")
      })
      navigate("/users");
    })
  }

  useEffect(() => {
    getRoles();
  }, []);


  return(
    <AppSidebar>
      { !loading && (
        <CreateUserForm roles={roles} />
      )}
    </AppSidebar>
  )
}
