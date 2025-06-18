<script setup lang="ts">
import { ref } from 'vue'

import { LOGO_PATH, APP_NAME, API_URL } from '@/customConfig'

import ForgotPassword from './ForgotPassword.vue'

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
import { useToast } from '@/components/ui/toast'
import { Input } from '@/components/ui/input'
import { Checkbox } from '@/components/ui/checkbox'
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import * as z from 'zod'

import axios from 'axios'
import { useI18n } from 'vue-i18n'
import router from '@/router'

const { t } = useI18n();
const { toast } = useToast();

const formSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50),
    password: z.string().min(5).max(50),
}))

const { handleSubmit } = useForm({
    validationSchema: formSchema,
})

const errorMessage = ref("")

const onSubmit = handleSubmit((values) => {

    axios.post(API_URL + '/login', {
        email: values.email,
        password: values.password
    })
    .then((response) => {
        sessionStorage.setItem("token", response.data.token);

        if(document.getElementById('remeber_me')?.ariaChecked === "true"){
            localStorage.setItem("token", response.data.token);
        }

        router.push("/");
    })
    .catch((error) => {
        if(error && error.status === 400) { // The credentials are not valid
            errorMessage.value = t("login.error.invalidCredentials");
        } else {
            errorMessage.value = t("login.error.other");
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
            <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left">{{ $t('login.title') }}</h2>
            <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

            <!-- Email field -->
            <FormField v-slot="{ componentField }" name="email">
                <FormItem>
                    <FormLabel>{{ $t('login.email') }}</FormLabel>
                    <FormControl>
                        <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <!-- Password field -->
            <FormField v-slot="{ componentField }" name="password">
                <FormItem>
                    <FormLabel>{{ $t('login.password') }}</FormLabel>
                    <FormControl>
                        <Input type="password" :placeholder="$t('login.password')" v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <!-- Remember me field -->
            <FormField v-slot="{ componentField }" name="remember_me">
                <FormItem>
                    <FormControl>
                        <div class="items-top flex gap-x-2">
                            <Checkbox id="remeber_me" v-bind="componentField" />
                            <div class="grid gap-1.5 leading-none">
                            <label
                                for="remeber_me"
                                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                            >
                                {{ $t('login.remember_me') }}
                            </label>
                            </div>
                        </div>
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>


            <Button type="submit">
                {{ $t('login.submit') }}
            </Button>

            <div class="mt-5">
                <ForgotPassword @response="(msg) => {
                    toast({
                        description: msg,
                    })
                }" />
            </div>

        </form>
    </main>
</template>