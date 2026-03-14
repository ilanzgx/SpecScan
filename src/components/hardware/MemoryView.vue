<script setup lang="ts">
import type { MemoryInfo, PhysicalMemorySlot } from "../../types/hardware";

defineProps<{
  memory: MemoryInfo | null;
  slots: PhysicalMemorySlot[] | null;
  formatBytes: (bytes: number) => string;
}>();
</script>

<template>
  <div class="p-4 space-y-6">
    <h2 class="text-2xl font-bold">Memória RAM</h2>

    <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
      <h3 class="text-xl font-bold mb-2">Uso Geral</h3>
      <ul v-if="memory" class="space-y-1">
        <li><strong>Total:</strong> {{ formatBytes(memory.total_memory) }}</li>
        <li><strong>Em uso:</strong> {{ formatBytes(memory.used_memory) }}</li>
        <li>
          <strong>Disponível:</strong> {{ formatBytes(memory.free_memory) }}
        </li>
      </ul>
      <p v-else>Carregando memória...</p>
    </div>

    <div v-if="slots && slots.length > 0" class="space-y-4">
      <h2 class="text-xl font-bold">Pentes Instalados</h2>
      <div
        v-for="slot in slots"
        :key="slot.part_number"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
        <h4 class="font-bold border-b border-white/10 pb-1 mb-2">
          Slot: {{ slot.capacity_gb }} GB - {{ slot.manufacturer }}
        </h4>
        <ul class="space-y-1 text-sm">
          <li>
            <strong>Tipo / Formato:</strong> {{ slot.memory_type }}
            {{ slot.form_factor }}
          </li>
          <li><strong>Velocidade:</strong> {{ slot.speed_mhz }} MHz</li>
          <li>
            <strong>Tensão Operacional:</strong> {{ slot.configured_voltage }} V
          </li>
          <li><strong>Part Number:</strong> {{ slot.part_number }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>
