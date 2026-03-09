<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const systemInfo = ref<any>(null);
const cpuInfo = ref<any>(null);
const memoryInfo = ref<any>(null);
const motherboardInfo = ref<any>(null);

onMounted(async () => {
  systemInfo.value = await invoke("get_system_info");
  cpuInfo.value = await invoke("get_cpu_info");
  memoryInfo.value = await invoke("get_memory_info");
  motherboardInfo.value = await invoke("get_motherboard_info");
});

const formatMemory = (bytes: number) => {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  if (bytes === 0) return "n/a";
  const i = parseInt(String(Math.floor(Math.log(bytes) / Math.log(1024))));
  return Math.round(bytes / Math.pow(1024, i)) + " " + sizes[i];
};
</script>

<template>
  <h1>SpecScan</h1>
  <h2 v-if="systemInfo">{{ systemInfo.os_name }}</h2>
  <h2 v-if="motherboardInfo">{{ motherboardInfo.manufacturer }}</h2>
  <h2 v-if="motherboardInfo">{{ motherboardInfo.product }}</h2>
  <h2 v-if="motherboardInfo">{{ motherboardInfo.version }}</h2>
  <h2 v-if="cpuInfo">
    {{ cpuInfo.name }} - {{ cpuInfo.cores }} cores - {{ cpuInfo.frequency }} MHz
  </h2>
  <h2 v-if="memoryInfo">
    {{ formatMemory(memoryInfo.total_memory) }} de memória RAM
  </h2>
  <h2 v-else>Carregando...</h2>
</template>
