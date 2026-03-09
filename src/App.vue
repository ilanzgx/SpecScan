<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  SystemInfo,
  CpuInfo,
  MemoryInfo,
  MotherboardInfo,
} from "./types/hardware";

const systemInfo = ref<SystemInfo | null>(null);
const cpuInfo = ref<CpuInfo | null>(null);
const memoryInfo = ref<MemoryInfo | null>(null);
const motherboardInfo = ref<MotherboardInfo | null>(null);

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
  <div class="flex flex-col gap-4">
    <!-- so -->
    <div>
      <h2 class="text-2xl">Sistema Operacional</h2>
      <h4 v-if="systemInfo">{{ systemInfo.os_name }}</h4>
    </div>

    <!-- cpu -->
    <div>
      <h2 class="text-2xl">CPU</h2>
      <h4 v-if="cpuInfo">
        {{ cpuInfo.name }} - {{ cpuInfo.cores }} cores -
        {{ cpuInfo.frequency }} MHz
      </h4>
    </div>

    <!-- memory -->
    <div>
      <h2 class="text-2xl">Memória</h2>
      <h4 v-if="memoryInfo">
        {{ formatMemory(memoryInfo.total_memory) }} de memória RAM
      </h4>
    </div>

    <!-- motherboard -->
    <div v-if="motherboardInfo">
      <h2 class="text-2xl">Placa Mãe</h2>
      <div>
        <h4>{{ motherboardInfo.manufacturer }}</h4>
        <h4>{{ motherboardInfo.product }}</h4>
        <h4>{{ motherboardInfo.version }}</h4>
      </div>
      <div>
        <h4>{{ motherboardInfo.serial_number }}</h4>
        <h4>{{ motherboardInfo.asset_tag }}</h4>
        <h4>{{ motherboardInfo.bios.vendor }}</h4>
        <h4>{{ motherboardInfo.bios.version }}</h4>
        <h4>{{ motherboardInfo.bios.release_date }}</h4>
      </div>

      <div
        class="gap-2"
        v-for="memorySlot in motherboardInfo.memory_slots"
        :key="memorySlot.slot_label"
      >
        <div class="w-3/12">
          <h4>{{ memorySlot.slot_label }}</h4>
          <h4>{{ memorySlot.bank_label }}</h4>
          <h4>{{ memorySlot.manufacturer }}</h4>
          <h4>{{ memorySlot.serial_number }}</h4>
          <h4>{{ memorySlot.part_number }}</h4>
          <h4>{{ memorySlot.size_mb }}</h4>
          <h4>{{ memorySlot.speed_mhz }}</h4>
          <h4>{{ memorySlot.memory_type }}</h4>
          <h4>{{ memorySlot.form_factor }}</h4>
          <h4>{{ memorySlot.configured_voltage }}</h4>
        </div>
      </div>
    </div>
    <div v-else>
      <h2>Placa Mãe</h2>
      <h4>Carregando...</h4>
    </div>
  </div>
</template>
