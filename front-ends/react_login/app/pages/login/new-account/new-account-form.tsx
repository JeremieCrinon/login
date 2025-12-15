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

export function NewAccountForm() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const formSchema = z.object({
    newEmail: z
      .email({ error: t("zod.email") }),
    newPassword: z
      .string()
      .min(8, { error: t("zod.min", { min: 8 }) })
      .regex(/[a-z]/, { error: t("zod.lowercase") })
      .regex(/[A-Z]/, { error: t("zod.uppercase") })
      .regex(/[0-9]/, { error: t("zod.number") })
      .regex(/[^a-zA-Z0-9]/, { error: t("zod.special") }),
    confirmPassword: z.string(),
  })
    .refine((data) => data.newPassword === data.confirmPassword, {
      error: t("zod.password_confirm"),
      path: ["confirmPassword"],
    })

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      newEmail: sessionStorage.getItem("user_email") ?? "",
      newPassword: "",
      confirmPassword: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    axios.post(`${API_URL}/modify-new-account/${t("locale")}`, {
      new_email: data.newEmail,
      new_password: data.newPassword
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("new_account.success.title"), {
          description: t("new_account.success.desc")
        })

        navigate("/logout");
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
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('new_account.title')}</CardTitle>
        <CardDescription>{t('new_account.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="new-account-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup>
            <Controller
              name="newEmail"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="new-account-form-new-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="new-account-form-new-email"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="newPassword"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="new-account-form-new-password">
                    {t('new_account.new_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="new-account-form-new-password"
                    type="password"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="confirmPassword"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="new-account-form-confirm-password">
                    {t('new_account.confirm_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="new-account-form-confirm-password"
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
        <Button type="submit" form="new-account-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

