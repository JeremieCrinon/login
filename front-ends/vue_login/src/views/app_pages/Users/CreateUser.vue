<script setup lang="ts">

import { ref } from 'vue';
import router from '@/router';

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
  DialogClose,
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
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { useToast } from '@/components/ui/toast'
import { Checkbox } from '@/components/ui/checkbox'
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import * as z from 'zod'

import axios from 'axios'
import { useI18n } from 'vue-i18n'

const { t, availableLocales } = useI18n();
const { toast } = useToast();

const emit = defineEmits(['user_change'])
const token = sessionStorage.getItem('token');

const formSchema = toTypedSchema(z.object({
    email: z.string().min(5).max(50),
    lang: z.string({ message: t("create_user.lang.error") }),
}))

const { handleSubmit } = useForm({
    validationSchema: formSchema,
})

const errorMessage = ref("");
const loading = ref(true);
const roles = ref<string[]>([]);

const onSubmit = handleSubmit((values) => {

    let rolesInputs = document.querySelectorAll("#roles button");

    let roles:string[] = [];

    rolesInputs.forEach((role) => role?.ariaChecked !== "false" ? roles.push(role.id) : "")

    // axios.post(API_URL + '/users' + values.lang, {
    axios.post(`${API_URL}/users/new/${values.lang}`, {
        email: values.email,
        roles: roles
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("create_user.confirm"),
        });
        emit('user_change');
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else if (error && error.response.status === 409) {
            errorMessage.value = t("create_user.errors.existing_mail");
        } else {
            errorMessage.value = t("create_user.errors.other");
        }
    })

});

const getRoles = () => {
    loading.value = true;

    axios.get(API_URL + `/users/list-roles`, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        roles.value = response.data.roles;
        filterRoles();
        loading.value = false;
    })
    .catch((error) => {
        console.error("Failed to get the possible roles for the users, redirectecting to /.");
        router.push('/'); // We redirect to the origin if there is an error, has we don't want a user here if we don't have the users list
        loading.value = false;
    })
}

const filterRoles = () => {
    roles.value = roles.value.filter((role) => role !== "user");
}

getRoles();

</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button>{{ $t("create_user.trigger") }}</Button>
        </DialogTrigger>
        <DialogContent>
            <form class="form space-y-3 mt-5 mb-5" @submit="onSubmit">
                <DialogHeader>
                    <DialogTitle>{{ $t("create_user.title") }}</DialogTitle>
                </DialogHeader>

                <p class="text-sm font-medium text-destructive" id="errorMessage">{{ errorMessage }}</p>

                <!-- Email field -->
                <FormField v-slot="{ componentField }" name="email">
                    <FormItem>
                        <FormLabel>{{ $t('create_user.email') }}</FormLabel>
                        <FormControl>
                            <Input type="text" placeholder="email@mail.com" v-bind="componentField" /><!-- no type email cause we don't want the browser's error displaying -->
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <div id="roles">
                    <div class="flex items-center space-x-2 mt-2" v-for="role in roles">
                        <Checkbox :id="role" />
                        <label :for="role" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            {{ role }}
                        </label>
                    </div>
                </div>

                <FormField v-slot="{ componentField }" name="lang">
                    <FormItem>
                        <FormLabel>{{ $t("create_user.lang.title") }}</FormLabel>

                        <Select v-bind="componentField">
                        <FormControl>
                            <SelectTrigger>
                                <SelectValue :placeholder="$t('create_user.lang.trigger')" />
                            </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                            <SelectGroup>
                                <SelectItem v-for="locale in $i18n.availableLocales" :value="locale" :key="locale">
                                    {{ locale }}
                                </SelectItem>
                            </SelectGroup>
                        </SelectContent>
                        </Select>
                        <FormDescription>
                            {{ $t("create_user.lang.description") }}
                        </FormDescription>
                        <FormMessage />
                    </FormItem>
                </FormField>

                
                <DialogFooter>
                    <Button type="submit">
                        {{ $t('create_user.submit') }}
                    </Button>
                </DialogFooter>
                
            </form>
        </DialogContent>
    </Dialog>
  </template>