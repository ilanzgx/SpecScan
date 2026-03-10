<script setup lang="ts">
import type { CpuInfo } from "../../types/hardware";

defineProps<{
  cpu: CpuInfo | null;
}>();
</script>

<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold mb-4">Processador</h2>
    <ul v-if="cpu" class="space-y-2">
      <li><strong>Modelo:</strong> {{ cpu.name }}</li>
      <li><strong>Núcleos:</strong> {{ cpu.cores }}</li>
      <li><strong>Frequência:</strong> {{ cpu.frequency }} MHz</li>
    </ul>
    <p v-else>Carregando CPU...</p>
  </div>
</template>
