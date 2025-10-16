<script setup lang="ts">

import { ref } from 'vue';

import { APP_NAME, LOGO_PATH, API_URL } from '@/customConfig';

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

const emit = defineEmits(['redirect_to_logout']);

const formSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50).email(),
}));

const { handleSubmit } = useForm({
    validationSchema: formSchema,
    initialValues: {
        email: sessionStorage.getItem('user_email') ?? "", // Pre-fill the email
    },
})

const errorMessage = ref("");
const change_email = ref(false);

const onSubmit = handleSubmit((values) => {
    const token = sessionStorage.getItem("token");

    axios.post(API_URL + '/edit-email/' + t('locale'), {
        new_email: values.email,
        password: "" // The API requires the password field, even in the case where the user hasn't a verified email (which is the case here). But just sending an empty string is enough.
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: change_email.value ? t("edit_unverified_email.confirm") : t("edit_unverified_email.resend_confirm"),
        })
        change_email.value && emit("redirect_to_logout");
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The user does not have a valid token
            emit("redirect_to_logout");
        } else if (error && error.response.status === 400 && error.response.data.error_code === "email_already_exists") {
            errorMessage.value = t("edit_unverified_email.errors.existing_mail");
        } else {
            errorMessage.value = t("edit_unverified_email.errors.other");
        }
    })

})

</script>

<template>
    <Button class="" @click="change_email = !change_email">{{ $t("edit_unverified_email.trigger") }}</Button>

    <form class="form space-y-3 w-5/6 self-center md:self-auto md:w-3/5" @submit="onSubmit" >
        
        <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

        <div :class="change_email ? '' : 'hidden'">
            <!-- Email field -->
            <FormField v-slot="{ componentField }" name="email">
                <FormItem>
                    <FormLabel>{{ $t('edit_unverified_email.email') }}</FormLabel>
                    <FormControl>
                        <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>
        </div>

        <Button type="submit">
            {{ change_email ? $t('edit_unverified_email.submit') : $t('edit_unverified_email.submit_new_email') }}
        </Button>
    </form>

</template>
