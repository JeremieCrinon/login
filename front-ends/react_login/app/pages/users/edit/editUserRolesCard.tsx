import { useState } from "react";
import { toast } from "sonner";
import axios from "axios";
import { useNavigate } from "react-router";

import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Checkbox } from "~/components/ui/checkbox";
import { Label } from "~/components/ui/label";
import { useTranslation } from "react-i18next";
import type { User } from "~/types/user";

import { API_URL } from "~/customConfig";

export function EditUserRolesCard({roles, user}: {roles: string[]; user: User}) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  function submit(e: React.FormEvent) {
    e.preventDefault();

    // Get the list of the chosen roles as they are not in the zod form
    const htmlRoles = document.getElementById("edit-user-form-roles")!.childNodes;
    const htmlRolesArray = Array.from(htmlRoles!);
    let chosenRoles: String[] = [];

    htmlRolesArray.forEach((htmlRole) => {
      const checkbox = htmlRole!.firstChild;
      //@ts-ignore // For some reason TS displays an error as it thinks dataset does not exist
      const checked = checkbox!.dataset.state == "checked";
      //@ts-ignore
      const role = checkbox!.dataset.role;

      if (checked) chosenRoles.push(role); 
    })

    axios.put(`${API_URL}/users/${user.id}/roles`, {
      roles: chosenRoles,
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("users.edit.roles.success.title"), {
          description: t("users.edit.roles.success.desc")
        })

        navigate("/users");
      })
      .catch((error) => {
        if (error.status == 401) {
          navigate("/");
        } else {
          console.error(error);
          setError(t("error.unknown"));
        }
      });
  }

 
  return (
    <Card className="w-full sm:max-w-md">
      <CardHeader className="text-center">
        <CardTitle>{t('users.edit.roles.title')}</CardTitle>
        <CardDescription>{t('users.edit.roles.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="edit-user-roles-form" onSubmit={submit}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}

          <div id="edit-user-form-roles" className="space-y-1 mt-5">

            {roles.map((role, i) => (
              <div key={i} className="flex items-center gap-3">
                <Checkbox id={`edit-user-form-role-${role}`} data-role={role} defaultChecked={user.roles.includes(role)} />
                <Label htmlFor={`edit-user-form-role-${role}`}>{role}</Label>
              </div>
            ))}

          </div>

        </form>
      </CardContent>
      <CardFooter>
        <Button type="submit" form="edit-user-roles-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

