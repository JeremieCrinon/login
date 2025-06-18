<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Icon } from '@iconify/vue'
import { useColorMode } from '@vueuse/core'

// Pass { disableTransition: false } to enable transitions
const mode = useColorMode()

import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline">
        <Icon icon="radix-icons:moon" class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <Icon icon="radix-icons:sun" class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span class="sr-only">{{ t("switch_color.button_span") }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end">
      <DropdownMenuItem @click="mode = 'light'">
        {{ t("switch_color.light") }}
      </DropdownMenuItem>
      <DropdownMenuItem @click="mode = 'dark'">
        {{ t("switch_color.dark") }}
      </DropdownMenuItem>
      <DropdownMenuItem @click="mode = 'auto'">
        {{ t("switch_color.system") }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>