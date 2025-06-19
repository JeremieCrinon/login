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


const users = ref<User[]>([]);
const total_users = ref(0);
const loading = ref(true);

const getUsers = () => {
    loading.value = true;

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

getUsers();


const user_email = sessionStorage.getItem("user_email");
</script>

<template>
    <Menu>

        <div class="mt-10">
            <CreateUser @user_change="() => getUsers()" />
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
                    <TableRow v-for="n in 10" :key="n" >
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
                            <EditUser v-if="user.id !== 1 && user.email !== user_email" :id="user.id" :user="user" @user_change="() => getUsers()" />
                        </TableCell>
                        <TableCell>
                            <DeleteUser v-if="user.id !== 1 && user.email !== user_email" :id="user.id" @user_change="() => getUsers()" />
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

  </Menu>
</template>