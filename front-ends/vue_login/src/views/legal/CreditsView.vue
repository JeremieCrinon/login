<script setup lang="ts">
import { LOGO_PATH, APP_NAME } from '@/customConfig';

import router from '@/router'

import { ChevronLeft } from 'lucide-vue-next';
</script>

<template>
    <div class="absolute left-0 top-0" @click="() => router.push('/')">
        <ChevronLeft />
    </div>
    <main class="flex md:flex-row flex-col w-full">
        <section class="w-full md:w-1/3 mt-5 flex flex-col items-center">
            <img class="w-1/3 sm:w-1/5 md:w-1/2" :src="LOGO_PATH" alt="logo" />
            <h1 class="font-main font-bold text-4xl md:text-3xl text-center mt-5">{{ APP_NAME }}</h1>
        </section>
        <div class="space-y-3 w-5/6 self-center md:self-auto md:w-3/5 mt-5 mb-5">
            <h2 class="font-main font-bold text-3xl md:text-2xl text-center md:text-left">{{ $t('credits.title') }}</h2>
            
            <p>{{ $t('credits.jeremie') }}</p>
            <p>{{ $t('credits.lucide') }}</p>
            <p>{{ $t('credits.symfony') }}</p>
            <p>{{ $t('credits.vue') }}</p>
            <p>{{ $t('credits.shadcn') }}</p>
            <p>{{ $t('credits.shadcnVue') }}</p>
            <p>{{ $t('credits.tailwind') }}</p>
        </div>
    </main>
</template>