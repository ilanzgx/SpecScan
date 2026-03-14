<script setup lang="ts">
import { ref, computed } from "vue";
import { useHardwareData } from "./composables/useHardwareData";
import AppSidebar from "./components/layout/AppSidebar.vue";
import SystemView from "./components/hardware/SystemView.vue";
import CpuView from "./components/hardware/CpuView.vue";
import GpuView from "./components/hardware/GpuView.vue";
import MemoryView from "./components/hardware/MemoryView.vue";
import DisksView from "./components/hardware/DisksView.vue";
import MotherboardView from "./components/hardware/MotherboardView.vue";
import NetworkView from "./components/hardware/NetworkView.vue";

const {
  isLoading,
  systemInfo,
  cpuInfo,
  cpuBenchmark,
  memoryInfo,
  motherboardInfo,
  disksInfo,
  physicalDisksInfo,
  physicalMemoryInfo,
  gpuInfo,
  networkInfo,
  networkAdaptersInfo,
  formatBytes,
  formatUptime,
} = useHardwareData();

const activeTab = ref("system");

const uptime = computed(() =>
  systemInfo.value ? formatUptime(systemInfo.value.uptime_seconds) : "—",
);
</script>

<template>
  <div class="app-shell">
    <AppSidebar v-model:activeTab="activeTab" />

    <main class="app-content">
      <!-- Loading state -->
      <div v-if="isLoading" class="loading-screen">
        <span class="loading-spinner" />
        <p class="loading-text">Lendo hardware…</p>
      </div>

      <!-- Views -->
      <template v-else>
        <SystemView
          v-if="activeTab === 'system'"
          :system="systemInfo"
          :cpu="cpuInfo"
          :cpuBenchmark="cpuBenchmark"
          :gpu="gpuInfo?.[0] || null"
          :memory="memoryInfo"
          :memorySlots="physicalMemoryInfo"
          :physicalDisks="physicalDisksInfo"
          :motherboard="motherboardInfo"
          :network="networkInfo"
          :formatBytes="formatBytes"
          :uptime="uptime"
        />
        <CpuView
          v-else-if="activeTab === 'cpu'"
          :cpu="cpuInfo"
          :cpuBenchmark="cpuBenchmark"
        />
        <GpuView v-else-if="activeTab === 'gpu'" :gpus="gpuInfo" />
        <MemoryView
          v-else-if="activeTab === 'memory'"
          :memory="memoryInfo"
          :slots="physicalMemoryInfo"
          :formatBytes="formatBytes"
        />
        <DisksView
          v-else-if="activeTab === 'disks'"
          :disks="disksInfo"
          :physicalDisks="physicalDisksInfo"
        />
        <MotherboardView
          v-else-if="activeTab === 'motherboard'"
          :motherboard="motherboardInfo"
        />
        <NetworkView
          v-else-if="activeTab === 'network'"
          :adapters="networkAdaptersInfo"
          :interfaces="networkInfo"
          :formatBytes="formatBytes"
        />
      </template>
    </main>
  </div>
</template>
