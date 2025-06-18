<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'

import { LOGO_PATH, APP_NAME, API_URL } from '@/customConfig'

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
import { useToast } from '@/components/ui/toast'
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import * as z from 'zod'

import axios from 'axios'
import { useI18n } from 'vue-i18n'
import router from '@/router'

const { t } = useI18n();
const { toast } = useToast();
const route = useRoute();

const formSchema = toTypedSchema(z.object({
    password: z
        .string()
        .min(8)
        .max(50)
        .regex(/[A-Z]/, { message: t("reset_password.password.no_uppercase") })
        .regex(/[a-z]/, { message: t("reset_password.password.no_lowercase") })
        .regex(/\d/, { message: t("reset_password.password.no_number") })
        .regex(/[\W_]/, { message: t("reset_password.password.no_special_char") }),
    password_confirm: z.string().min(8).max(50),
})
.refine((data) => data.password === data.password_confirm, {
    message: t("reset_password.password.confirm_not_matching"),
    path: ["password_confirm"],
}));

const { handleSubmit } = useForm({
    validationSchema: formSchema,
})

const errorMessage = ref("");

const code = route.params.code

const onSubmit = handleSubmit((values) => {
    axios.post(API_URL + '/reset-password', {
        new_password: values.password,
        code: route.params.code,
    })
    .then((response) => {
        toast({
            description: t("reset_password.confirm"),
        })
        router.push("/");
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The credentials are not valid
            errorMessage.value = t("reset_password.error.bad_code");
        } else {
            errorMessage.value = t("reset_password.error.other");
        }
    })

})

if(sessionStorage.getItem('token')){
    router.push("/"); // if the token is set in the sessionStorage, we send the user to the / route, so we verify if the token is correct
}

if(localStorage.getItem('token')){
    router.push("/"); // if the token is set in the localStorage, we send the user to the / route, so it handle it
}
</script>

<template>

    <main class="flex md:flex-row flex-col w-full">
        <section class="w-full md:w-1/3 mt-5 flex flex-col items-center">
            <img class="w-1/3 sm:w-1/5 md:w-1/2" :src="LOGO_PATH" alt="logo" />
            <h1 class="font-main font-bold text-4xl md:text-3xl text-center mt-5">{{ APP_NAME }}</h1>
        </section>
        <form class="form space-y-3 w-5/6 self-center md:self-auto md:w-3/5 mt-5 mb-5" @submit="onSubmit">
            <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left">{{ $t('reset_password.title') }}</h2>
            <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

            <div class="flex flex-row w-full">
                <!-- Password field -->
                <FormField v-slot="{ componentField }" name="password">
                    <FormItem>
                        <FormLabel>{{ $t('reset_password.new_password') }}</FormLabel>
                        <FormControl>
                            <Input type="password" :placeholder="$t('reset_password.new_password')" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <div class="ml-5"></div> <!-- Not really a good practice, but I cannot add classes to the <FormField> elements -->
                <FormField v-slot="{ componentField }" name="password_confirm" class="w-1/2">
                    <FormItem>
                        <FormLabel>{{ $t('reset_password.confirm_password') }}</FormLabel>
                        <FormControl>
                            <Input type="password" :placeholder="$t('reset_password.confirm_password')" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
            </div>


            <Button type="submit">
                {{ $t('reset_password.submit') }}
            </Button>

        </form>
    </main>
</template>