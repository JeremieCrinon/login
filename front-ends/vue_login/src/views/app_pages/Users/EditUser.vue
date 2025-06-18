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

// Lucid imports
import { User, UserPen } from 'lucide-vue-next';

import axios from 'axios'
import { useI18n } from 'vue-i18n'

const { t, availableLocales } = useI18n();
const { toast } = useToast();

interface User {
    id: number;
    email: string;
    roles: string[];
}

const emit = defineEmits(['user_change']);
const props = defineProps<{
    user: User
}>();

const token = sessionStorage.getItem('token');

const emailErrorMessage = ref("");
const rolesErrorMessage = ref("");
const loading = ref(true);
const roles = ref<string[]>([]);

const emailFormSchema = toTypedSchema(z.object({
    new_email: z.string().email(t("edit_user.email.invalid_email")),
    lang: z.string({ message: t("edit_user.email.lang.error") }),
}));

const { handleSubmit: handleEmailSubmit } = useForm({
    validationSchema: emailFormSchema,
})

const onEmailSubmit = handleEmailSubmit((values) => {
    const token = sessionStorage.getItem("token");

    axios.put(`${API_URL}/users/${props.user.id}/email/${values.lang}`, {
        email: values.new_email,
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("edit_user.email.confirm"),
        })
        emailErrorMessage.value = "";
        emit('user_change');
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else if (error && error.response.status === 409) {
            emailErrorMessage.value = t("edit_user.email.errors.existing_email");
        } else {
            emailErrorMessage.value = t("edit_user.email.errors.other");
        }
    })

})

const handleRolesSubmit = () => {
    let rolesInputs = document.querySelectorAll("#roles button");

    let roles:string[] = [];

    rolesInputs.forEach((role) => role?.ariaChecked !== "false" ? roles.push(role.id) : "");

    roles.push("user");

    axios.put(`${API_URL}/users/${props.user.id}/roles`, {
        roles: roles,
    }, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("edit_user.roles.confirm"),
        })
        rolesErrorMessage.value = "";
        emit('user_change');
    })
    .catch((error) => {
        if(error && error.response.status === 401) { // The user does not have a valid token
            router.push("/logout");
        } else {
            rolesErrorMessage.value = t("edit_user.roles.errors.other");
        }
    })
}

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
            <Button variant="secondary">
                <UserPen />
            </Button>
        </DialogTrigger>
        <DialogContent>
            
            <DialogHeader>
                <DialogTitle>{{ user && $t("edit_user.title", {email: user.email}) }}</DialogTitle>
            </DialogHeader>

            
            <!-- Email form -->
            <form class="form space-y-3 w-full mt-5" @submit="onEmailSubmit" >
                <p class="text-lg font-bold">{{ $t('edit_user.email.title') }}</p>
                <p class="text-sm font-medium text-destructive" id="errorMessage">{{ emailErrorMessage }}</p>

                <!-- Email field -->
                <FormField v-slot="{ componentField }" name="new_email">
                    <FormItem>
                        <FormLabel>{{ $t('edit_user.email.new_email') }}</FormLabel>
                        <FormControl>
                            <Input type="email" :placeholder="$t('edit_user.email.new_email')" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>

                <FormField v-slot="{ componentField }" name="lang">
                    <FormItem>
                        <FormLabel>{{ $t("edit_user.email.lang.title") }}</FormLabel>

                        <Select v-bind="componentField">
                        <FormControl>
                            <SelectTrigger>
                                <SelectValue :placeholder="$t('edit_user.email.lang.trigger')" />
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
                            {{ $t("edit_user.email.lang.description") }}
                        </FormDescription>
                        <FormMessage />
                    </FormItem>
                </FormField>
                
                <Button type="submit">
                    {{ $t('edit_user.email.submit') }}
                </Button>
            </form>

            <!-- Roles form -->
            <div class="space-y-3 mt-5 mb-5">
                <p class="text-lg font-bold">{{ $t('edit_user.roles.title') }}</p>
                <p class="text-sm font-medium text-destructive">{{ rolesErrorMessage }}</p>

                <div id="roles">
                    <div class="flex items-center space-x-2 mt-2" v-for="role in roles">
                        <Checkbox :id="role" :default-checked="user.roles.includes(role)" />
                        <label :for="role" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            {{ role }}
                        </label>
                    </div>
                </div>

                <Button @click="handleRolesSubmit">
                    {{ $t('edit_user.roles.submit') }}
                </Button>
            </div>
            

        </DialogContent>
    </Dialog>
  </template>