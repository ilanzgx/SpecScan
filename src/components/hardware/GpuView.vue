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
      <div
        v-for="(gpu, i) in gpus"
        :key="gpu.name"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
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

        <div
          v-if="
            gpu.benchmark &&
            gpu.benchmark.ranking !== 'N/A' &&
            gpu.benchmark.ranking
          "
          class="mt-4 bg-cyan-500/10 border border-cyan-500/20 p-4 rounded-xl"
        >
          <h4
            class="font-semibold text-lg mb-4 border-b border-cyan-500/30 pb-2 text-cyan-400"
          >
            Benchmark (PassMark)
          </h4>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <span class="block text-gray-400 text-sm">G3D Score</span>
              <span class="font-mono text-xl text-white">{{
                gpu.benchmark.g3d_score
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">G2D Score</span>
              <span class="font-mono text-xl text-white">{{
                gpu.benchmark.g2d_score
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">Ranking Global</span>
              <span class="font-mono text-xl text-white"
                >#{{ gpu.benchmark.ranking }}</span
              >
            </div>
            <div>
              <span class="block text-gray-400 text-sm">Preço Est.</span>
              <span class="font-mono text-lg text-white">{{
                gpu.benchmark.price !== "NA" ? gpu.benchmark.price : "N/A"
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">Lançamento</span>
              <span class="font-mono text-lg text-white">{{
                gpu.benchmark.release_date
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">Clock</span>
              <span class="font-mono text-lg text-white">{{
                gpu.benchmark.core_clock !== "NA"
                  ? gpu.benchmark.core_clock
                  : "N/A"
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">TDP</span>
              <span class="font-mono text-lg text-white">{{
                gpu.benchmark.tdp !== "NA" ? gpu.benchmark.tdp + "W" : "N/A"
              }}</span>
            </div>
            <div>
              <span class="block text-gray-400 text-sm">Memória (Score)</span>
              <span class="font-mono text-lg text-white">{{
                gpu.benchmark.vram
              }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <p v-else>Nenhuma GPU encontrada ou carregando...</p>
  </div>
</template>
