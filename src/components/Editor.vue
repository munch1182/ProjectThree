<script setup lang="ts">
import loader from '@monaco-editor/loader';

import { onMounted, ref } from 'vue';
import api from './ApiFroMoanco';

const editor = ref()
onMounted(async () => {
    const monaco = await loader.init();
    api.register(monaco);

    const code = `
# get###

@BASEURL = "https://www.api.com/api/v1"
@user = {
    "username": "testuser1"
}

###

GET @BASEURL/login

ContentType: application/json

{
    "username": str
}

=>

{
    "userid": num
}

MOCK-REQ: @user
MOCK-RES:
{
    "userid": 1000001
}
###


###

###
    `

    const e = monaco.editor.create(editor.value, {
        value: code,
        language: api.NAME,
        theme: api.THEME,
        readOnly: false,
        lineNumbers: 'on',
    })
    api.addCommand(monaco, e);
})

</script>

<template>
    <div class="w-full h-full flex">
        <div ref="editor" class="flex-1"></div>
    </div>
</template>

<style scoped>

</style>