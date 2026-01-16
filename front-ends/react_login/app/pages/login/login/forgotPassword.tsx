import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import { toast } from "sonner";
import * as z from "zod";
import axios from "axios";
import { useTranslation } from "react-i18next";

// Shadcn/ui imports
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogClose,
} from "~/components/ui/dialog";
import { Button } from "~/components/ui/button";
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from "~/components/ui/field";
import { Input } from "~/components/ui/input";

import { API_URL } from "~/customConfig";

export function ForgotPassword() {
  const { t } = useTranslation();

  const [open, setOpen] = useState(false);
  const [error, setError] = useState("");

  const formSchema = z.object({
    email: z
      .email({ error: t("zod.email") }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: ""
    },
  });

  function onSubmit(data: z.infer<typeof formSchema>) {

    axios.post(`${API_URL}/forgot-password/${t("locale")}`, {
      email: data.email,
    })
      .then(() => {
        setError("");

        toast(t("login.forgot_password.success.title"), {
          description: t("login.forgot_password.success.desc")
        });
        setOpen(false);
      })
      .catch((error) => {
        console.error(error);
        setError(t("error.unknow"));
      });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger>
       {t("login.forgot_password.trigger")}
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {t("login.forgot_password.title")}
          </DialogTitle>
          <DialogDescription>
            {t("login.forgot_password.desc")}
          </DialogDescription>
        </DialogHeader>
        
        <form id="forgot-password-form" onSubmit={form.handleSubmit(onSubmit)}>
          {error && (
            <p className="text-sm font-medium text-destructive">{error}</p>
          )}
          
          <FieldGroup>
            <Controller
              name="email"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="forgot-password-form-email">
                    {t('email')}
                  </FieldLabel>
                  <Input
                    {...field}
                    id="forgot-password-form-email"
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

        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="secondary">{t("cancel")}</Button>
          </DialogClose>
          <Button type="submit" form="forgot-password-form">
           {t("submit")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>  
  )
}
