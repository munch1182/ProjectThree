<script setup lang="ts">
import { ref } from "vue";
import { testStartTime } from "../../common/api";

const show = ref("")
testStartTime()
    .then(r => show.value = r.toString())
    .catch(e => console.log(e))

</script>

<template>
    {{ show }}
</template>

<style scoped>

</style>