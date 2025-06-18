<script setup lang="ts">

import Menu from '@/components/Menu.vue';

import ModifyAccountEmail from './ModifyAccountEmail.vue';
import ModifyAccountPassword from './ModifyAccountPassword.vue';

</script>

<template>
  <Menu>
    <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left mt-5">{{ $t('modify_account.title') }}</h2>

    <!-- <h1>Work In Progress....</h1> -->

    <!-- Email form -->
    <ModifyAccountEmail />

    <!-- Password form -->
    <ModifyAccountPassword />

  </Menu>
</template>