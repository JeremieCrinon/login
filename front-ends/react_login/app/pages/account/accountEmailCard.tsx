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

export function EditAccountEmailCard() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const formSchema = z.object({
    email: z
      .email({ error: t("zod.email") }),
    password: z
      .string()
      .min(3, { error: t("zod.min", { min: 3 }) }),
   })

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: sessionStorage.getItem("user_email") ?? "",
      password: "",
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    axios.post(`${API_URL}/edit-email/${t('locale')}`, {
      new_email: data.email,
      password: data.password,
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("account.email.success.title"), {
          description: t("account.email.success.desc")
        })

        navigate("/logout");
      })
      .catch((error) => {
        if (error.status == 401) {
          setError(t("account.error.bad_password"))
        } else if (error.status === 409) {
          setError(t("account.error.email_taken"))
        } else {
          console.error(error);
          setError(t("error.unknown"));
        }
      });
  }

 
  return (
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('account.email.title')}</CardTitle>
      </CardHeader>

      <CardContent>
        <form id="account-email-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup className="mt-5">
            <Controller
              name="email"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="account-form-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="account-form-email"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

          <Controller
            name="password"
            control={form.control}
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="account-form-email-password">
                  {t('password')}
                </FieldLabel>
                <Input
                  {...field}
                  id="account-form-email-password"
                  type="password"
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
        <Button className="ml-5" type="submit" form="account-email-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

