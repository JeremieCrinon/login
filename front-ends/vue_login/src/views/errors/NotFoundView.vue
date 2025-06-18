<script setup lang="ts">
import { LOGO_PATH, APP_NAME } from '@/customConfig'

// ShadCn imports
import { Button } from '@/components/ui/button'


import { useI18n } from 'vue-i18n'
import router from '@/router'

const { t } = useI18n();

</script>

<template>

    <main class="flex md:flex-row flex-col w-full">
        <section class="w-full md:w-1/3 mt-5 flex flex-col items-center">
            <img class="w-1/3 sm:w-1/5 md:w-1/2" :src="LOGO_PATH" alt="logo" />
            <h1 class="font-main font-bold text-4xl md:text-3xl text-center mt-5">{{ APP_NAME }}</h1>
        </section>
        <section class="w-full md:w-2/3 mt-20 flex flex-col items-center">
            <h2 class="text-center font-main font-bold text-3xl md:text-2xl">{{ $t("not_found.title") }}</h2>
            <p class="text-center font-main text-lg mt-2 mb-5" v-html="t('not_found.description')"></p>
            <RouterLink to="/">
                <Button >{{ $t("not_found.link") }}</Button>
            </RouterLink>
        </section>
    </main>

</template>