import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import { toast } from "sonner";
import * as z from "zod";
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
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from "~/components/ui/field";
import { Input } from "~/components/ui/input";
import { useTranslation } from "react-i18next";

import { API_URL } from "~/customConfig";

export function CreateUserForm({roles}: {roles: String[]}) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const formSchema = z.object({
    email: z
      .email({ error: t("zod.email") }),
  })

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    // axios.post(`${API_URL}/modify-new-account/${t("locale")}`, {
    //   new_email: data.newEmail,
    //   new_password: data.newPassword
    // }, {
    //   headers: {
    //     "Authorization": `Bearer ${token}`
    //   }
    // })
    //   .then(() => {
    //     setError("");
    //
    //     toast(t("new_account.success.title"), {
    //       description: t("new_account.success.desc")
    //     })
    //
    //     navigate("/logout");
    //   })
    //   .catch((error) => {
    //     if (error.status == 401) {
    //       navigate("/");
    //     } else {
    //       console.error(error);
    //       setError(t("error.unknown"));
    //     }
    //   });
  }

 
  return (
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('users.create.title')}</CardTitle>
        <CardDescription>{t('users.create.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="create-user-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup>
            <Controller
              name="email"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="create-user-form-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="create-user-form-email"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>
        </form>
      </CardContent>
      <CardFooter>
        <Button variant="secondary" onClick={() => navigate("/users")}>{t("cancel")}</Button>
        <Button className="ml-5" type="submit" form="create-user-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

