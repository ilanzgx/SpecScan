<script setup lang="ts">
import type { GpuInfo } from "../../types/hardware";

defineProps<{
  gpus: GpuInfo[] | null;
}>();
</script>

<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold mb-4">Placa de Vídeo / GPU</h2>
    <div v-if="gpus && gpus.length > 0" class="space-y-6">
      <div v-for="(gpu, i) in gpus" :key="gpu.name">
        <h3 class="text-xl font-bold mb-2">GPU {{ i + 1 }}: {{ gpu.name }}</h3>
        <ul class="space-y-1">
          <li><strong>Fabricante:</strong> {{ gpu.manufacturer }}</li>
          <li>
            <strong>Processador de Vídeo:</strong> {{ gpu.video_processor }}
          </li>
          <li><strong>VRAM:</strong> {{ gpu.adapter_ram_gb }} GB</li>
          <li><strong>DAC:</strong> {{ gpu.adapter_dac_type }}</li>
          <li>
            <strong>Driver:</strong> {{ gpu.driver_version }} ({{
              gpu.driver_date
            }})
          </li>
          <li><strong>Resolução:</strong> {{ gpu.video_mode_description }}</li>
          <li>
            <strong>Taxa de Atualização:</strong> {{ gpu.refresh_rate }}Hz ({{
              gpu.min_refresh_rate
            }}–{{ gpu.max_refresh_rate }}Hz)
          </li>
          <li>
            <strong>Profundidade de Cor:</strong> {{ gpu.bits_per_pixel }} bpp
          </li>
        </ul>
      </div>
    </div>
    <p v-else>Nenhuma GPU encontrada ou carregando...</p>
  </div>
</template>
