<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const systemInfo = ref<any>(null);
const cpuInfo = ref<any>(null);

onMounted(async () => {
  systemInfo.value = await invoke("get_system_info");
  cpuInfo.value = await invoke("get_cpu_info");
});
</script>

<template>
  <h1>SpecScan</h1>
  <h2 v-if="systemInfo">{{ systemInfo.os_name }}</h2>
  <h2 v-if="cpuInfo">
    {{ cpuInfo.name }} - {{ cpuInfo.cores }} cores - {{ cpuInfo.frequency }} MHz
  </h2>
  <h2 v-else>Carregando...</h2>
</template>
