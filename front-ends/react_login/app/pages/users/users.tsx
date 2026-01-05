import { requireRole } from "~/lib/auth";
import { AppSidebar } from "~/components/sidebar";
import axios from "axios";
import { API_URL } from "~/customConfig";
import { useState, useEffect } from "react";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import UsersTable from "./usersTable";
import type {User} from "~/types/user"

export function clientLoader() {
  return requireRole("user");
}

export default function Users() {
  const token = sessionStorage.getItem("token");
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);

  const navigate = useNavigate();
  const { t } = useTranslation();

  function getUsers() {
    setLoading(true);

    axios.get(`${API_URL}/users`, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
    .then((r) => {
      setUsers(r.data.users);
      setLoading(false);
      console.log(r.data.users);
    })
    .catch((e) => {
      console.error(e);
      toast(t("users.fetch.error.title"), {
        description: t("users.fetch.error.description")
      })
      navigate("/");
    })
  }

  useEffect(() => {
    getUsers();
  }, []);

  return (
    <AppSidebar>
      {!loading && (
        <UsersTable users={users}/>
      )}
    </AppSidebar>
  );
}
