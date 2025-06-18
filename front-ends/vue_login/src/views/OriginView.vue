<script setup lang="ts">
    import router from '@/router';
    import axios from 'axios'

    import { API_URL } from '@/customConfig';

    const verifyToken = async (token: string): Promise<Boolean> => {
      try {
        const response = await axios.get(API_URL + "/user-infos", {
            headers: { Authorization: "Bearer " + token }
        });

        if (response.data.result === null) {
          throw new Error();
        }
        sessionStorage.setItem("token", token);
        sessionStorage.setItem("roles", JSON.stringify(response.data.roles));
        sessionStorage.setItem("user_email", response.data.user_mail);
        
        return true; // Token is valid
      } catch (error) {
          return false; // Token is invalid
      }
    }

    if(sessionStorage.getItem("token") || localStorage.getItem("token")){
        const token = sessionStorage.getItem("token") ? sessionStorage.getItem("token") : localStorage.getItem("token")
        
        if (!token) {
          router.push("login");
        }
        
        token && verifyToken(token)
        .then(result => {
          if (result) { // If result is true, it means that the user is connected
            router.push("home");
          } else { // It means the user does not have a valid token
            sessionStorage.removeItem("token"); // We remove it so it does not try this token again, if it's not valid now, it wont be later
            localStorage.removeItem("token");
            router.push("login");
          }
        })
        .catch(() => {
          router.push("login");
        })
    } else {
        router.push("login")
    }


</script>
<template>
  {{ $t("loading") }}
</template>
