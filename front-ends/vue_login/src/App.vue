<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';

import Toaster from '@/components/ui/toast/Toaster.vue';

import SwitchColor from './components/SwitchColor.vue';
</script>

<template>
  <!-- A button to switch theme (dark / light / auto) always displayed on the top right -->
  <div class="absolute top-2 right-2">
    <SwitchColor />
  </div>

  <RouterView />

  <!-- Two links always displayed on the bottom left -->
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
