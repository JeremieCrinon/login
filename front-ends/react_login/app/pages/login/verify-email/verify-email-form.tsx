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

export function VerifyEmailForm() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const formSchema = z.object({
    code: z
      .string()
  })

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      code: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {
    axios.post(`${API_URL}/verify-email`, {
      code: data.code,
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("verify_email.success.title"), {
          description: t("verify_email.success.desc")
        })

        navigate("/");
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
        <CardTitle>{t('verify_email.title')}</CardTitle>
        <CardDescription>{t('verify_email.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="verify-email-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup>
            <Controller
              name="code"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="verify-email-form-code">
                    {t('code')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="verify-email-form-code"
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
        <Button type="submit" form="verify-email-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

