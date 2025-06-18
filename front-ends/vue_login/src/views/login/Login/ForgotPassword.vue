<script setup lang="ts">

import { ref } from 'vue';

import { API_URL } from '@/customConfig';

// ShadCn imports
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogClose
} from '@/components/ui/dialog'
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

import axios from 'axios'
import { useI18n } from 'vue-i18n'
import router from '@/router'

const { t } = useI18n();
const emit = defineEmits(['response']);

const formSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50),
}))

const { handleSubmit } = useForm({
    validationSchema: formSchema,
})

const errorMessage = ref("")

const onSubmit = handleSubmit((values) => {

    // axios.post(API_URL + '/api/forgot/password/' + t("locale"), {
    axios.post(`${API_URL}/forgot-password/${t("locale")}`, {
        email: values.email
    })
    .then((response) => {
        emit('response', t("forgot_password.confirm"))
    })
    .catch((error) => {
        errorMessage.value = t("forgot_password.errors.other");
    })

})

</script>

<template>
    <Dialog>
      <DialogTrigger>{{ $t("forgot_password.trigger") }}</DialogTrigger>
      <DialogContent>
        <form class="form space-y-3 mt-5 mb-5" @submit="onSubmit">
            <DialogHeader>
                <DialogTitle>{{ $t("forgot_password.title") }}</DialogTitle>
            </DialogHeader>

            <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

            <!-- Email field -->
            <FormField v-slot="{ componentField }" name="email">
                <FormItem>
                    <FormLabel>{{ $t('forgot_password.email') }}</FormLabel>
                    <FormControl>
                        <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <DialogFooter>
                <DialogClose as-child>
                    <Button type="submit">
                        {{ $t('forgot_password.submit') }}
                    </Button>
                </DialogClose>
            </DialogFooter>
            
        </form>
      </DialogContent>
    </Dialog>
  </template>