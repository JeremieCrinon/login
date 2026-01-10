import { useEffect, useState } from "react";
import { requireRole } from "~/lib/auth";
import { EditUserForm } from "./editUserForm";
import { AppSidebar } from "~/components/sidebar";
import axios from "axios";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router";
import { toast } from "sonner";

import { API_URL } from "~/customConfig";
import type { User } from "~/types/user";

export function clientLoader() {
  return requireRole("edit_users");
}

export default function editUser() {
  const token = sessionStorage.getItem("token");
  const [roles, setRoles] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<User | null>(null);

  const { t } = useTranslation();
  const navigate = useNavigate();
  const params = useParams();

  function getRoles() {
    setLoading(true);
    axios.get(`${API_URL}/users/list-roles`, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
    .then((r) => {
      setRoles(r.data.roles);
      getUser();
    })
    .catch((e) => {
      console.error(e);

      toast(t("users.create.error.roles.title"), {
        description: t("users.create.error.roles.description")
      })
      navigate("/users");
    })
  }

  function getUser() {
    setLoading(true);
    axios.get(`${API_URL}/users/${params.id}`, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
    .then((r) => {
      setUser(r.data);
      setLoading(false);
    })
    .catch((e) => {
      console.error(e);

      toast(t("users.edit.error.user.title"), {
        description: t("users.edit.error.user.description")
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
        <EditUserForm user={user!} roles={roles} />
      )}
    </AppSidebar>
  )
}
