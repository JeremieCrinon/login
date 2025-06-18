<script setup lang="ts">

import { API_URL } from '@/customConfig';

import { ref, warn, watch } from 'vue'
import router from '@/router';

import Menu from '@/components/Menu.vue';
import DeleteUser from './DeleteUser.vue';
import CreateUser from './CreateUser.vue';
import EditUser from './EditUser.vue';

import axios from 'axios';

// ShadCn imports
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Skeleton } from '@/components/ui/skeleton'
import {
  Button,
} from '@/components/ui/button'
import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from '@/components/ui/pagination'
import { Label } from '@/components/ui/label'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@/components/ui/number-field'
import { Input } from '@/components/ui/input'

import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const token = sessionStorage.getItem('token');

// The structure of the users
interface User {
    id: number;
    email: string;
    roles: string[];
}

const per_page = ref(parseInt(localStorage.getItem("users_per_page") ?? "10"));
const page = ref(1);
const users = ref<User[]>([]);
const total_users = ref(0);
const loading = ref(true);
const filter_email = ref('');

const getUsers = (per_page: number = 10, page: number = 1) => {
    loading.value = true;
    per_page = per_page < 1 ? 1 : per_page; // Just to be sure that it is not inferior to 1
    page = page < 1 ? 1 : page; // Just to be sure that it is not inferior to 1

    // axios.get(API_URL + `/users?per_page=${per_page}&page=${page}${filter_email.value ? '&email=' + filter_email.value : ''}`, {
    axios.get(API_URL + `/users`, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        users.value = filterUsers(response.data.users);
        total_users.value = response.data.total_users;
        loading.value = false;
    })
    .catch((error) => {
        router.push('/'); // We redirect to the origin if there is an error, has we don't want a user here if we don't have the users list
        loading.value = false;
    })
}

const filterUsers = (users: Array<User>) => {
  users = users.map((user) => ({
    ...user,
    roles: user.roles.filter((role) => role !== "user"),
  }));

//   users = users.filter((user) => user.email !== sessionStorage.getItem("user_email"));

  return users;
};

getUsers(per_page.value, page.value);

watch(per_page, () => {
    localStorage.setItem('users_per_page', per_page.value.toString());
    getUsers(per_page.value, page.value);
});
watch(page, () => getUsers(per_page.value, page.value));
watch(filter_email, () => getUsers(per_page.value, page.value));

const user_email = sessionStorage.getItem("user_email");
</script>

<template>
    <Menu>

        <div class="mt-10">
            <CreateUser @user_change="() => getUsers(per_page, page)" />
        </div>
    
        <div class="flex mt-5">
            <!-- The filed for users per page -->
            <NumberField :model-value="per_page" @update:model-value="(newValue) => per_page = newValue" :min="1" :max="100">
                <Label>{{ $t("users.table.per_page") }}</Label>
                <NumberFieldContent>
                <NumberFieldDecrement />
                <NumberFieldInput />
                <NumberFieldIncrement />
                </NumberFieldContent>
            </NumberField>

            <!-- The input to filter users by email -->
            <div class="grid w-full max-w-sm items-center gap-1.5 ml-5">
                <Label for="filter_email">{{ $t("users.table.filter_email") }}</Label>
                <Input v-model="filter_email" id="filter_email" type="text" />
            </div>
        </div>

        <!-- The user's table itself -->
        <div v-if="loading" class="mb-5">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>{{ $t("users.table.email") }}</TableHead>
                        <TableHead>{{ $t("users.table.roles") }}</TableHead>
                        <TableHead>{{ $t("users.table.edit") }}</TableHead>
                        <TableHead>{{ $t("users.table.delete") }}</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow v-for="n in per_page" :key="n" >
                        <TableCell>
                            <Skeleton class="w-48 h-7" />
                        </TableCell>
                        <TableCell>
                            <Skeleton class="w-48 h-7" />
                        </TableCell>
                        <TableCell>
                            <Skeleton class="w-12 h-7" />
                        </TableCell>
                        <TableCell>
                            <Skeleton class="w-12 h-7" />
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
        <div v-else class="mb-5">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>{{ $t("users.table.email") }}</TableHead>
                        <TableHead>{{ $t("users.table.roles") }}</TableHead>
                        <TableHead>{{ $t("users.table.edit") }}</TableHead>
                        <TableHead>{{ $t("users.table.delete") }}</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow v-for="user in users" :key="user.id">
                        <TableCell class="font-medium">
                            {{ user.email }}
                        </TableCell>
                        <TableCell>
                            {{ user.roles.join(', ') }}
                        </TableCell>
                        <TableCell>
                            <EditUser v-if="user.id !== 1 && user.email !== user_email" :id="user.id" :user="user" @user_change="() => getUsers(per_page, page)" />
                        </TableCell>
                        <TableCell>
                            <DeleteUser v-if="user.id !== 1 && user.email !== user_email" :id="user.id" @user_change="() => getUsers(per_page, page)" />
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <!-- Pagination -->
            <Pagination v-slot="{ page }" :total="(total_users / per_page) * 10" show-edges :default-page="page" @update:page="(newPage) => page = newPage">
                <PaginationList v-slot="{ items }" class="flex items-center gap-1">
                    <PaginationFirst />
                    <PaginationPrev />

                    <template v-for="(item, index) in items">
                        <PaginationListItem v-if="item.type === 'page'" :key="index" :value="item.value" as-child>
                        <Button class="w-10 h-10 p-0" :variant="item.value === page ? 'default' : 'outline'">
                            {{ item.value }}
                        </Button>
                        </PaginationListItem>
                        <PaginationEllipsis v-else :key="item.type" :index="index" />
                    </template>

                    <PaginationNext />
                    <PaginationLast />
                </PaginationList>
            </Pagination>
        </div>

  </Menu>
</template>