<script setup lang="ts">

import { ref } from 'vue';

import { APP_NAME, LOGO_PATH, API_URL } from '@/customConfig';

import router from '@/router';
import axios from 'axios';

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

import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const { toast } = useToast();


const formSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50).email(),
    password: z
        .string()
        .min(8)
        .max(50)
        .regex(/[A-Z]/, { message: t("new_account.password.no_uppercase") })
        .regex(/[a-z]/, { message: t("new_account.password.no_lowercase") })
        .regex(/\d/, { message: t("new_account.password.no_number") })
        .regex(/[\W_]/, { message: t("new_account.password.no_special_char") }),
    password_confirm: z.string().min(8).max(50),
})
.refine((data) => data.password === data.password_confirm, {
    message: t("new_account.password.confirm_not_matching"),
    path: ["password_confirm"],
}));

const { handleSubmit } = useForm({
    validationSchema: formSchema,
    initialValues: {
        email: sessionStorage.getItem('user_email') ?? "", // Pre-fill the email
        password: "",
        password_confirm: "",
    },
})

const errorMessage = ref("")

const onSubmit = handleSubmit((values) => {
    const token = sessionStorage.getItem("token");

    // axios.post(API_URL + '/modify-new-account' + t('locale'), {
    axios.post(`${API_URL}/modify-new-account/${t('locale')}`, {
        new_email: values.email,
        new_password: values.password
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("new_account.confirm"),
        })
        router.push("/logout"); // Everything went fine, we redirect the user to logout with a toaster message to let him/her know that he/she has to login again
    })
    .catch((error) => {
        console.log(error.response.data.error );
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else if (error && error.response.status === 400 && error.response.data.error_code === "email_already_exists") {
            errorMessage.value = t("new_account.errors.existing_mail");
        } else {
            errorMessage.value = t("new_account.errors.other");
        }
    })

})


// We verify that the user has the role NEW_ACCOUNT, and redirect him to the origin if does not
const userRoles = JSON.parse(sessionStorage.getItem('roles') || '[]');

if (!userRoles.includes("new_account")) {
    router.push('/');
}

</script>

<template>

    <main class="flex md:flex-row flex-col w-full">
        <section class="w-full md:w-1/3 mt-5 flex flex-col items-center">
            <img class="w-1/3 sm:w-1/5 md:w-1/2" :src="LOGO_PATH" alt="logo" />
            <h1 class="font-main font-bold text-4xl md:text-3xl text-center mt-5">{{ APP_NAME }}</h1>
        </section>
        <form class="form space-y-3 w-5/6 self-center md:self-auto md:w-3/5 mt-5 mb-5" @submit="onSubmit" >
            <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left">{{ $t('new_account.title') }}</h2>
            <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

            <!-- Email field -->
            <FormField v-slot="{ componentField }" name="email">
                <FormItem>
                    <FormLabel>{{ $t('new_account.email') }}</FormLabel>
                    <FormControl>
                        <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <div class="flex flex-row w-full">
                <!-- Password field -->
                <FormField v-slot="{ componentField }" name="password">
                    <FormItem>
                        <FormLabel>{{ $t('new_account.new_password') }}</FormLabel>
                        <FormControl>
                            <Input type="password" :placeholder="$t('new_account.new_password')" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <div class="ml-5"></div> <!-- Not really a good practice, but I cannot add classes to the <FormField elements -->
                <FormField v-slot="{ componentField }" name="password_confirm" class="w-1/2">
                    <FormItem>
                        <FormLabel>{{ $t('new_account.confirm_password') }}</FormLabel>
                        <FormControl>
                            <Input type="password" :placeholder="$t('new_account.confirm_password')" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
            </div>

            <Button type="submit">
                {{ $t('new_account.submit') }}
            </Button>
        </form>
    </main>

</template>