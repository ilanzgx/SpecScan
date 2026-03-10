<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  SystemInfo,
  CpuInfo,
  MemoryInfo,
  MotherboardInfo,
  DiskInfo,
  PhysicalDiskInfo,
  PhysicalMemorySlot,
} from "./types/hardware";

const systemInfo = ref<SystemInfo | null>(null);
const cpuInfo = ref<CpuInfo | null>(null);
const memoryInfo = ref<MemoryInfo | null>(null);
const motherboardInfo = ref<MotherboardInfo | null>(null);
const disksInfo = ref<DiskInfo[] | null>(null);
const physicalDisksInfo = ref<PhysicalDiskInfo[] | null>(null);
const physicalMemoryInfo = ref<PhysicalMemorySlot[] | null>(null);

onMounted(async () => {
  systemInfo.value = await invoke("get_system_info");
  cpuInfo.value = await invoke("get_cpu_info");
  memoryInfo.value = await invoke("get_memory_info");
  motherboardInfo.value = await invoke("get_motherboard_info");
  disksInfo.value = await invoke("get_disks_info");
  physicalDisksInfo.value = await invoke("get_physical_disks_info");
  physicalMemoryInfo.value = await invoke("get_physical_memory_info");
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
      <h4 v-if="systemInfo">
        {{ systemInfo.os_name }} {{ systemInfo.os_version }}
      </h4>
      <h4 v-if="systemInfo">Kernel Version {{ systemInfo.kernel_version }}</h4>
      <h4 v-if="systemInfo">{{ systemInfo.host_name }}</h4>
      <h4 v-if="systemInfo">{{ systemInfo.cpu_arch }}</h4>
      <h4 v-if="systemInfo">PC ligado a: {{ systemInfo.uptime_seconds }}s</h4>
      <h4 v-if="systemInfo">{{ systemInfo.boot_time_seconds }}</h4>
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
      <h2 class="text-2xl">Memória RAM</h2>

      <div v-if="memoryInfo" class="mb-4">
        <h3 class="text-lg font-bold">Uso Geral:</h3>
        <h4>Total: {{ formatMemory(memoryInfo.total_memory) }}</h4>
        <h4>Usado: {{ formatMemory(memoryInfo.used_memory) }}</h4>
        <h4>Livre: {{ formatMemory(memoryInfo.free_memory) }}</h4>
      </div>

      <div v-if="physicalMemoryInfo && physicalMemoryInfo.length > 0">
        <h3 class="text-lg font-bold">Pentes Físicos:</h3>
        <div
          v-for="ram in physicalMemoryInfo"
          :key="ram.part_number"
          class="gap-2 mb-1"
        >
          <h4>
            {{ ram.capacity_gb }}GB {{ ram.manufacturer }}
            {{ ram.memory_type }} @ {{ ram.speed_mhz }}MHz
            {{ ram.form_factor }} | P/N: {{ ram.part_number }} |
            {{ ram.configured_voltage }}V
          </h4>
        </div>
      </div>
    </div>

    <!-- disks -->
    <div>
      <h2 class="text-2xl">Discos</h2>

      <!-- Hardware -->
      <div
        v-if="physicalDisksInfo && physicalDisksInfo.length > 0"
        class="mb-4"
      >
        <h3 class="text-lg font-bold">Drives Físicos:</h3>
        <div
          v-for="disk in physicalDisksInfo"
          :key="disk.serial_number"
          class="gap-2 mb-1"
        >
          <h4>
            {{ disk.model }} - {{ disk.size_gb }} GB (S/N:
            {{ disk.serial_number }})
          </h4>
        </div>
      </div>

      <!-- Partições -->
      <div v-if="disksInfo && disksInfo.length > 0">
        <h3 class="text-lg font-bold">Volumes Lógicos:</h3>
        <div v-for="disk in disksInfo" :key="disk.name" class="gap-2 mb-2">
          <h4>
            {{ disk.name }} ({{ disk.mount_point }}) [{{ disk.kind }} -
            {{ disk.file_system }}]
            <span v-if="disk.is_removable">(Removível)</span>
          </h4>
          <h4>
            {{ disk.usage_percent }}% em uso ({{ disk.used_gb }} GB usados de
            {{ disk.total_gb }} GB) - Livre: {{ disk.available_gb }} GB
          </h4>
        </div>
      </div>
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
        <h4>
          S/N: {{ motherboardInfo.serial_number }} | Asset:
          {{ motherboardInfo.asset_tag }} | BIOS:
          {{ motherboardInfo.bios.vendor }} (v{{
            motherboardInfo.bios.version
          }}
          - {{ motherboardInfo.bios.release_date }})
        </h4>
      </div>

      <div
        class="gap-2"
        v-for="memorySlot in motherboardInfo.memory_slots"
        :key="memorySlot.slot_label"
      >
        <h1 class="text-lg">Slot {{ memorySlot.slot_label }}</h1>
        <div class="">
          <h4>
            {{ memorySlot.bank_label }}: {{ memorySlot.size_mb }}MB
            {{ memorySlot.manufacturer }} {{ memorySlot.memory_type }} @
            {{ memorySlot.speed_mhz }}MHz {{ memorySlot.form_factor }} | P/N:
            {{ memorySlot.part_number }} | S/N: {{ memorySlot.serial_number }} |
            {{ memorySlot.configured_voltage }}
          </h4>
        </div>
      </div>
    </div>
    <div v-else>
      <h2>Placa Mãe</h2>
      <h4>Carregando...</h4>
    </div>
  </div>
</template>
