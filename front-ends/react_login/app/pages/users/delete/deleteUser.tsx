import { toast } from "sonner";
import axios from "axios";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "~/components/ui/alert-dialog";
import { Button } from "~/components/ui/button";
import { Trash2 } from "lucide-react";

import { API_URL} from "~/customConfig";

export function DeleteUser({id, email}: {id: number; email: string}) {
  const navigate = useNavigate();
  const { t } = useTranslation(); 

  const token = sessionStorage.getItem("token");

  function submit() {
    axios.delete(`${API_URL}/users/${id}`, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
    .then(() => {
      toast(t("users.delete.success.title"), {
        description: t("users.delete.success.desc")
      })

      //TODO: Send a message to the parent components to re-load the users list.
    })
    .catch((error) => {
      // Display a toast whatever the error is to inform the user there has been an error and the user isn't deleted
      toast(t("users.delete.error.title"), {
        description: t("users.delete.error.desc")
      })

      if (error.status == 401) {
        navigate("/");
      } else {
        console.error(error);
      }
    })
  }

  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="destructive">
          <Trash2 />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("users.delete.title")} {email}</AlertDialogTitle>
          <AlertDialogDescription>{t("users.delete.desc")}</AlertDialogDescription>
        </AlertDialogHeader>
        
        <AlertDialogFooter>
          <AlertDialogCancel>{t("cancel")}</AlertDialogCancel>
          <AlertDialogAction onClick={submit}>{t("confirm")}</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>

  )

}
















