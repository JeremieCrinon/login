<script setup lang="ts">

import { API_URL } from '@/customConfig';

// ShadCn imports
import { Button } from '@/components/ui/button'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import { useToast } from '@/components/ui/toast';

import axios from 'axios';

// Lucide imports
import { Trash2 } from 'lucide-vue-next';

import { useI18n } from 'vue-i18n'

const { t } = useI18n();
const { toast } = useToast();

const props = defineProps({
  id: Number
})
const emit = defineEmits(['user_change'])

const token = sessionStorage.getItem("token");

const deleteUser = () => {
    axios.delete(API_URL + `/users/${props.id}`, {
        headers: { Authorization: "Bearer " + token }
    })
    .then((response) => {
        toast({
            description: t("delete_user.success"),
        })
        emit('user_change');
    })
    .catch((error) => {
        toast({
            description: t("delete_user.error"),
        })
    })
}
</script>
<template>
    <AlertDialog>
        <AlertDialogTrigger>
            <Button variant="destructive">
                <Trash2 />
            </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>{{ $t("delete_user.title") }}</AlertDialogTitle>
                <AlertDialogDescription>
                {{ $t("delete_user.description") }}
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>{{ $t("delete_user.cancel") }}</AlertDialogCancel>
                <div @click="deleteUser">
                    <AlertDialogAction>{{ $t("delete_user.confirm") }}</AlertDialogAction>
                </div>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
</template>