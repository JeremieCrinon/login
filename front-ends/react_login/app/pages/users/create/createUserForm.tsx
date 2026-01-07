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
  FieldContent,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
  FieldTitle
} from "~/components/ui/field";
import { Checkbox } from "~/components/ui/checkbox";
import { Label } from "~/components/ui/label";
import { Input } from "~/components/ui/input";
import {
  RadioGroup,
  RadioGroupItem
} from "~/components/ui/radio-group";
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
    lang: z
      .string()
      .min(1, t("users.create.error.no_lang"))
  })

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      lang: "",
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {

    // Get the list of the chosen roles as they are not in the zod form
    const htmlRoles = document.getElementById("create-user-form-roles")!.childNodes;
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

    axios.post(`${API_URL}/users/new/${data.lang}`, {
      email: data.email,
      roles: chosenRoles
    }, {
      headers: {
        "Authorization": `Bearer ${token}`
      }
    })
      .then(() => {
        setError("");

        toast(t("users.create.success.title"), {
          description: t("users.create.success.desc")
        })

        navigate("/users");
      })
      .catch((error) => {
        if (error.status == 401) {
          navigate("/");
        } else if (error.status === 409) {
          setError(t("users.create.error.email_taken"))
        } else {
          console.error(error);
          setError(t("error.unknown"));
        }
      });
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

          <div id="create-user-form-roles" className="space-y-1 mt-5">

            {roles.map((role, i) => (
              <div key={i} className="flex items-center gap-3">
                <Checkbox id={`create-user-form-role-${role}`} data-role={role} />
                <Label htmlFor={`create-user-form-role-${role}`}>{role}</Label>
              </div>
            ))}

          </div>

          <FieldGroup className="mt-5">
            <Controller
              name="lang"
              control={form.control}
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
                        id="create-user-form-lang-en"
                        aria-invalid={fieldState.invalid}
                      />
                      <Label htmlFor="create-user-form-lang-en">en</Label>
                    </div>
                    
                    <div className="flex items-center gap-3">
                      <RadioGroupItem
                        value="fr"
                        id="create-user-form-lang-fr"
                        aria-invalid={fieldState.invalid}
                      />
                      <Label htmlFor="create-user-form-lang-fr">fr</Label>
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
        <Button className="ml-5" type="submit" form="create-user-form">
          {t('submit')}
        </Button>
      </CardFooter>
    </Card>
  )
}

