<script setup lang="ts">
import { ref } from 'vue';
import { ApiDoc, Api } from './apidoc'
import { getFromDir, saveToDir } from './apidocfun'

let apis = ref<ApiDoc[] | void>()

let api = new ApiDoc()
api.projectPath = "api1"

getFromDir()
    .then(a => apis.value = a)

// saveToDir([api])

</script>

<template>
    {{ apis }}
</template>

<style scoped>

</style>