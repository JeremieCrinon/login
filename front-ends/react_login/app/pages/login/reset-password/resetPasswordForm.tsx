import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import * as z from "zod";
import axios from "axios";
import { Controller, useForm } from "react-hook-form";

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

import { API_URL } from "~/customConfig";

export function ResetPasswordForm({code}: {code: string}) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  
  const formSchema = z.object({
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
      newPassword: "",
      confirmPassword: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    axios.post(`${API_URL}/reset-password`, {
      code: code,
      new_password: data.newPassword
    })
      .then(() => {
        setError("");

        toast(t("reset_password.success.title"), {
          description: t("reset_password.success.desc")
        })

        navigate("/");
      })
      .catch((e) => {
        if (e.status == 401) {
          setError(t("reset_password.error.code"));
        } else {
          console.error(e);
          setError(t("error.unknown"));
        }
      });
  }

   return (
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('reset_password.title')}</CardTitle>
      </CardHeader>

      <CardContent>
        <form id="reset-password-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup>
              <Controller
              name="newPassword"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="reset-password-form-new-password">
                    {t('reset_password.new_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="reset-password-form-new-password"
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
                  <FieldLabel htmlFor="reset-password-form-confirm-password">
                    {t('reset_password.confirm_password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="reset-password-form-confirm-password"
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
        <Button type="submit" form="reset-password-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )

}
