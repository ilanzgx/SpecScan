<script setup lang="ts">
import type { DiskInfo, PhysicalDiskInfo } from "../../types/hardware";

defineProps<{
  disks: DiskInfo[] | null;
  physicalDisks: PhysicalDiskInfo[] | null;
}>();
</script>

<template>
  <div class="p-4 space-y-6">
    <h2 class="text-2xl font-bold">Discos</h2>

    <div v-if="physicalDisks && physicalDisks.length > 0" class="space-y-4">
      <h3 class="text-xl font-bold">Drives Físicos</h3>
      <div
        v-for="disk in physicalDisks"
        :key="disk.serial_number"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
        <ul class="space-y-1 text-sm">
          <li><strong>Modelo:</strong> {{ disk.model }}</li>
          <li><strong>Capacidade:</strong> {{ disk.size_gb }} GB</li>
          <li><strong>Número de Série:</strong> {{ disk.serial_number }}</li>
        </ul>
      </div>
    </div>

    <div v-if="disks && disks.length > 0" class="space-y-4">
      <h3 class="text-xl font-bold">Volumes Lógicos (Partições)</h3>
      <div
        v-for="disk in disks"
        :key="disk.name"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
        <h4 class="font-bold border-b border-white/10 pb-1 mb-2">
          {{ disk.name }} ({{ disk.mount_point }})
        </h4>
        <ul class="space-y-1 text-sm">
          <li>
            <strong>Tipo:</strong> {{ disk.kind }} — {{ disk.file_system }}
          </li>
          <li>
            <strong>Uso:</strong> {{ disk.usage_percent }}% (Usado:
            {{ disk.used_gb }} GB | Disp: {{ disk.available_gb }} GB)
          </li>
          <li><strong>Tamanho Total:</strong> {{ disk.total_gb }} GB</li>
          <li v-if="disk.is_removable">
            <strong>Nota:</strong> Mídia Removível
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
