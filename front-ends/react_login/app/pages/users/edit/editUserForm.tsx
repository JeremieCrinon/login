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
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "~/components/ui/field";
import { Checkbox } from "~/components/ui/checkbox";
import { Label } from "~/components/ui/label";
import { Input } from "~/components/ui/input";
import {
  RadioGroup,
  RadioGroupItem
} from "~/components/ui/radio-group";
import { useTranslation } from "react-i18next";
import type { User } from "~/types/user";

import { API_URL } from "~/customConfig";

export function EditUserForm({user, roles}: {user: User; roles: string[]}) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const token = sessionStorage.getItem("token");

  const emailFormSchema = z.object({
    email: z
      .email({ error: t("zod.email") }),
    lang: z
      .string()
      .min(1, t("users.create.error.no_lang"))
  })

  const emailForm = useForm<z.infer<typeof emailFormSchema>>({
    resolver: zodResolver(emailFormSchema),
    defaultValues: {
      email: user.email,
    },
  });

  function onEmailSubmit(data: z.infer<typeof emailFormSchema>) {
    axios.put(`${API_URL}/users/${user.id}/email/${data.lang}`, {
      email: data.email,
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("users.edit.email.success.title"), {
          description: t("users.edit.email.success.desc")
        })

        navigate("/users");
      })
      .catch((error) => {
        if (error.status == 401) {
          navigate("/");
        } else if (error.status === 409) {
          setError(t("users.edit.error.email_taken"))
        } else {
          console.error(error);
          setError(t("error.unknown"));
        }
      });
  }

 
  return (
    <Card className="w-full sm:max-w-md absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
      <CardHeader className="text-center">
        <CardTitle>{t('users.edit.email.title')}</CardTitle>
        <CardDescription>{t('users.edit.email.desc')}</CardDescription>
      </CardHeader>

      <CardContent>
        <form id="edit-user-email-form" onSubmit={emailForm.handleSubmit(onEmailSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          <FieldGroup className="mt-5">
            <Controller
              name="email"
              control={emailForm.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="edit-user-form-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="edit-user-form-email"
                    data-invalid={fieldState.invalid}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>

          <FieldGroup className="mt-5">
            <Controller
              name="lang"
              control={emailForm.control}
              render={({ field, fieldState }) => (
                <FieldSet data-invalid={fieldState.invalid}>
                  <FieldLegend>{t("users.create.lang.title")}</FieldLegend>
                  <FieldDescription>{t("users.create.lang.desc")}</FieldDescription>
                  <RadioGroup
                    name={field.name}
                    value={field.value}
                    onValueChange={field.onChange}
                    aria-invalid={fieldState.invalid}
                  > 
                  <div className="flex items-center gap-3">
                      <RadioGroupItem
                        value="en"
                        id="edit-user-form-lang-en"
                        aria-invalid={fieldState.invalid}
                      />
                      <Label htmlFor="edit-user-form-lang-en">en</Label>
                    </div>
                    
                    <div className="flex items-center gap-3">
                      <RadioGroupItem
                        value="fr"
                        id="edit-user-form-lang-fr"
                        aria-invalid={fieldState.invalid}
                      />
                      <Label htmlFor="edit-user-form-lang-fr">fr</Label>
                    </div>
                  </RadioGroup>
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </FieldSet>
              )}
            />
          </FieldGroup>

        </form>
      </CardContent>
      <CardFooter>
        <Button variant="secondary" onClick={() => navigate("/users")}>{t("cancel")}</Button>
        <Button className="ml-5" type="submit" form="edit-user-email-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

