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

export function EditAccountPasswordCard() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const formSchema = z.object({
    password: z
      .string()
      .min(3, { error: t("zod.min", { min: 3 }) }),
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
      password: "",
      newPassword: "",
      confirmPassword: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    axios.post(`${API_URL}/edit-password`, {
      current_password: data.password,
      new_password: data.newPassword
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("account.password.success.title"), {
          description: t("account.password.success.desc")
        })

        navigate("/logout");
      })
      .catch((error) => {
        if (error.status == 401) {
          setError(t("account.error.bad_password"))
        } else {
          console.error(error);
          setError(t("error.unknown"));
        }
      });
  }

 
  return (
    <Card className="w-full sm:max-w-md ">
      <CardHeader className="text-center">
        <CardTitle>{t('account.password.title')}</CardTitle>
      </CardHeader>

      <CardContent>
        <form id="account-password-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup className="mt-5">
            <Controller
              name="password"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="account-form-current-password">
                    {t('account.password.current_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="account-form-current-password"
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
              name="newPassword"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="account-form-new-password">
                    {t('account.password.new_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="account-form-new-password"
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
                  <FieldLabel htmlFor="account-form-confirm-password">
                    {t('account.password.confirm_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="account-form-confirm-password"
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
        <Button className="ml-5" type="submit" form="account-password-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

