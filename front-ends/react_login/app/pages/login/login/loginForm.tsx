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
import { Checkbox } from "~/components/ui/checkbox";
import { Label } from "~/components/ui/label";
import { useTranslation } from "react-i18next";

import { API_URL } from "~/customConfig";
import { ForgotPassword } from "./forgotPassword";

export function LoginForm() {
  const { t } = useTranslation();

  const [error, setError] = useState("");
  const navigate = useNavigate();

  const formSchema = z.object({
    email: z
      .email({ error: t("zod.email") }),
    password: z
      .string()
      .min(3, { error: t("zod.min", { min: 3 }) }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {

    axios.post(`${API_URL}/login`, {
      email: data.email,
      password: data.password
    })
      .then((response) => {
        setError("");

        sessionStorage.setItem("token", response.data.token)

        const rememberMe = document.getElementById("login-form-remeber-me")?.ariaChecked;

        if (rememberMe === "true") localStorage.setItem("token", response.data.token);

        toast(t("login.success.title"))

        navigate("/");
      })
      .catch((error) => {
        if (error.status == 400) {
          setError(t("login.error.credentials"));
        } else {
          console.error(error);
          setError(t("error.unknow"));
        }
      });
  }

  return (
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('login.title')}</CardTitle>
        <CardDescription>{t('login.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="login-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup>
            <Controller
              name="email"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="login-form-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="login-form-email"
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
                  <FieldLabel htmlFor="login-form-password">
                    {t('password')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="login-form-password"
                    type="password"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}

            />

            <div className="flex items-start gap-3">
              <Checkbox id="login-form-remeber-me" />
              <Label htmlFor="login-form-remeber-me">{t('remember_me')}</Label>
            </div>

          </FieldGroup>
        </form>
      </CardContent>
      <CardFooter>
        <Button className="mr-5" type="submit" form="login-form">
          {t('submit')}
        </Button>
        <ForgotPassword />
      </CardFooter>
    </Card>
  )
}
