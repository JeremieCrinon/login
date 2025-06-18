<script setup lang="ts">

import router from '@/router';

localStorage.removeItem("token");
sessionStorage.removeItem("token");
sessionStorage.removeItem("roles");

router.push('/');

</script>
<template>
    Please wait....
</template>