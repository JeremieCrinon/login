<script setup lang="ts">
import { API_URL } from '@/customConfig';

import { ref } from 'vue';

import router from '@/router';
import axios from 'axios';

import Menu from '@/components/Menu.vue';

// ShadCn imports
import { Button } from '@/components/ui/button'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Checkbox } from '@/components/ui/checkbox'
import { useToast } from '@/components/ui/toast'
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import * as z from 'zod'

const passwordErrorMessage = ref("");

import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const { toast } = useToast();

const passwordFormSchema = toTypedSchema(z.object({
    old_password: z.string().min(1).max(50),
    new_password: z
        .string()
        .min(8)
        .max(50)
        .regex(/[A-Z]/, { message: t("modify_account.password.errors.no_uppercase") })
        .regex(/[a-z]/, { message: t("modify_account.password.errors.no_lowercase") })
        .regex(/\d/, { message: t("modify_account.password.errors.no_number") })
        .regex(/[\W_]/, { message: t("modify_account.password.errors.no_special_char") }),
    password_confirm: z.string().min(8).max(50),
})
.refine((data) => data.new_password === data.password_confirm, {
    message: t("modify_account.password.errors.confirm_not_matching"),
    path: ["password_confirm"],
}));

const { handleSubmit: handlePasswordSubmit } = useForm({
    validationSchema: passwordFormSchema,
})

const onPasswordSubmit = handlePasswordSubmit((values) => {
    const token = sessionStorage.getItem("token");

    axios.post(API_URL + '/edit-password', {
        current_password: values.old_password,
        new_password: values.new_password
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("modify_account.password.confirm"),
        })
        passwordErrorMessage.value = "";
        router.push("/");
    })
    .catch((error) => {
        if(error && error.response.status === 401) {
            passwordErrorMessage.value = t("modify_account.errors.bad_old_password");
        } else {
            passwordErrorMessage.value = t("modify_account.errors.other");
        }
    })

})

</script>

<template>
    <form class="form space-y-3 w-full mt-5 mb-5" @submit="onPasswordSubmit" >
        <p class="text-sm font-medium text-destructive" id="errorMessage">{{ passwordErrorMessage }}</p>

        <FormField v-slot="{ componentField }" name="old_password">
            <FormItem>
                <FormLabel>{{ $t('modify_account.password.old_password') }}</FormLabel>
                <FormControl>
                    <Input type="password" :placeholder="$t('modify_account.password.old_password')" v-bind="componentField" />
                </FormControl>
                <FormMessage />
            </FormItem>
        </FormField>

        <div class="flex flex-row w-full">
            <!-- Password field -->
            <FormField v-slot="{ componentField }" name="new_password">
                <FormItem>
                    <FormLabel>{{ $t('modify_account.password.new_password') }}</FormLabel>
                    <FormControl>
                        <Input type="password" :placeholder="$t('modify_account.password.new_password')" v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>
            <div class="ml-5"></div> <!-- Not really a good practice, but I cannot add classes to the <FormField elements -->
            <FormField v-slot="{ componentField }" name="password_confirm" class="w-1/2">
                <FormItem>
                    <FormLabel>{{ $t('modify_account.password.confirm_password') }}</FormLabel>
                    <FormControl>
                        <Input type="password" :placeholder="$t('modify_account.password.confirm_password')" v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>
        </div>
        
        <Button type="submit">
            {{ $t('modify_account.password.submit') }}
        </Button>
    </form>
</template>