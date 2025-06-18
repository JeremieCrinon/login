<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';

import Toaster from '@/components/ui/toast/Toaster.vue';

import SwitchColor from './components/SwitchColor.vue';
</script>

<template>
  <div class="absolute top-2 right-2">
    <SwitchColor />
  </div>

  <RouterView />

  <div class="absolute bottom-2 left-2 flex z-50">
    <div>
      <RouterLink to="/privacy">{{ $t("privacy.link") }}</RouterLink>
    </div>
    <div class="ml-2">
      <RouterLink to="/credits">{{ $t("credits.link") }}</RouterLink>
    </div>
  </div>

  <Toaster /> <!-- The toaster component from ShadCn, displayed only when there is a message to display -->
</template>
