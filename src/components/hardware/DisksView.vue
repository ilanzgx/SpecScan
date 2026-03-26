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
      <h3 class="text-xl font-bold mb-4">Volumes Lógicos (Partições)</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          v-for="disk in disks"
          :key="disk.name"
          class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl flex flex-col"
        >
          <h4 class="font-bold border-b border-white/10 pb-2 mb-3">
            {{ disk.name }} ({{ disk.mount_point }})
          </h4>
          <ul class="space-y-2 text-sm">
            <li>
              <strong>Tipo:</strong> {{ disk.kind }} — {{ disk.file_system }}
            </li>
            <li>
              <strong>Uso:</strong> {{ disk.usage_percent }}% (Usado:
              {{ disk.used_gb }} GB | Disp: {{ disk.available_gb }} GB)
            </li>
            <li><strong>Tamanho Total:</strong> {{ disk.total_gb }} GB</li>
            <li v-if="disk.is_removable" class="text-yellow-400 mt-1">
              <strong>Nota:</strong> Mídia Removível
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>
