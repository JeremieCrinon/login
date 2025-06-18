<script setup lang="ts">

import { ref } from 'vue';

import { APP_NAME, LOGO_PATH, API_URL } from '@/customConfig';

import router from '@/router';
import axios from 'axios';

import EditUnverifiedEmail from './EditUnverifiedEmail.vue';

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
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import * as z from 'zod'
import { useI18n } from 'vue-i18n';

const { t } = useI18n();


const formSchema = toTypedSchema(z.object({
    code: z.string(),
}));

const { handleSubmit } = useForm({
    validationSchema: formSchema,
})

const errorMessage = ref("")

const onSubmit = handleSubmit((values) => {
    const token = sessionStorage.getItem("token");

    axios.post(API_URL + '/verify-email', {
        code: values.code,
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        router.push("/"); // Everything went fine, we redirect the user to the origin so his/her roles are updated and he/she is redirected to where he/she should be
    })
    .catch((error) => {
        console.log(error);
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else if (error && error.response.status === 400) {
            errorMessage.value = (t("verify_email.errors.invalid_code"));
        } else {
            errorMessage.value = t("verify_email.errors.other");
        }
    })

})


// We verify that the user has the role UNVERIFIED_EMAIL, and redirect him to the origin if does not
const userRoles = JSON.parse(sessionStorage.getItem('roles') || '[]');

if (!userRoles.includes("unverified_email")) {
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
            <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left">{{ $t('verify_email.title') }}</h2>
            <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

            <!-- Code field -->
            <FormField v-slot="{ componentField }" name="code">
                <FormItem>
                    <FormLabel>{{ $t('verify_email.code') }}</FormLabel>
                    <FormControl>
                        <Input type="text" v-bind="componentField" />
                    </FormControl>
                    <FormDescription>
                        {{ $t('verify_email.code_description') }}
                    </FormDescription>
                    <FormMessage />
                </FormItem>
            </FormField>

            <Button type="submit">
                {{ $t('verify_email.submit') }}
            </Button>

            <div class="pt-5">
                <EditUnverifiedEmail @redirect_to_logout="() => router.push('/logout')" />
            </div>
            
        </form>
    </main>

    <section class="ml-10 mt-10">
        
    </section>

</template>