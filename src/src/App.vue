<script setup lang="ts">
import Content from "./components/Content.vue";
import BottomStatus from "./components/BootomStatus.vue";
</script>

<template>
  <div class="flex  flex-col-reverse h-screen">
    <footer>
      <BottomStatus />
    </footer>
    <article class="flex flex-col flex-grow w-full">
      <Content />
    </article>

  </div>
</template>