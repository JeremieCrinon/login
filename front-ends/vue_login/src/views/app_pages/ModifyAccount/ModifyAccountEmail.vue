<script setup lang="ts">

import { API_URL } from '@/customConfig';

import { ref } from 'vue';

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

const emailErrorMessage = ref("");

import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const { toast } = useToast();

const emailFormSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50).email(),
    password: z.string()
}));

const { handleSubmit: handleEmailSubmit } = useForm({
    validationSchema: emailFormSchema,
    initialValues: {
        email: sessionStorage.getItem('user_email') ?? "", // Pre-fill the email
    },
})

const onEmailSubmit = handleEmailSubmit((values) => {
    const token = sessionStorage.getItem("token");

    // axios.post(API_URL + '/edit-email' + t('locale'), {
    axios.post(`${API_URL}/edit-email/${t("locale")}`, {
        new_email: values.email,
        password: values.password
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("modify_account.email.confirm"),
        })
        router.push("/logout"); // Everything went fine, we redirect the user to logout with a toaster message to let him/her know that he/she has to login again
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else if (error && error.response.status === 409) {
            emailErrorMessage.value = t("modify_account.errors.existing_mail");
        } else {
            emailErrorMessage.value = t("modify_account.errors.other");
        }
    })

});
</script>
<template>
    <form class="form space-y-3 w-full mb-5" @submit="onEmailSubmit" >
        <p class="text-sm font-medium text-destructive" id="errorMessage">{{ emailErrorMessage }}</p>

        <div class="w-48 md:w-96">
            <!-- Email field -->
            <FormField v-slot="{ componentField }" name="email">
                <FormItem>
                    <FormLabel>{{ $t('modify_account.email.email') }}</FormLabel>
                    <FormControl>
                        <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <!-- Password field -->
            <FormField v-slot="{ componentField }" name="password">
                <FormItem>
                    <FormLabel>{{ $t('modify_account.email.password') }}</FormLabel>
                    <FormControl>
                        <Input type="password" :placeholder="$t('modify_account.email.password')" v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>
        </div>
        
        <Button type="submit">
            {{ $t('modify_account.email.submit') }}
        </Button>
    </form>
</template>