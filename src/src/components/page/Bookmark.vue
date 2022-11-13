<script setup lang="ts">
import PartInputWrapper from '../PartInputWrapper.vue';
import { bookmark } from '../../common/api';

//接收PartInputWrapper传入的输入
function onInput(value: string) {
    console.log(value)
    bookmark(value)
        .then(() => { console.log("ok"); })
        .catch((e) => { console.log("error:", e); })
}
</script>

<template>
    <PartInputWrapper @input="onInput">
    </PartInputWrapper>
</template>

<style scoped>

</style>