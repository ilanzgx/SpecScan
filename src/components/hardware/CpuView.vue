<script setup lang="ts">
import { onMounted } from "vue";
import type { CpuInfo } from "../../types/hardware";

const props = defineProps<{
  cpu: CpuInfo | null;
}>();

onMounted(() => {
  console.log(props.cpu);
});
</script>

<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold mb-4">Processador (CPU)</h2>
    <div v-if="cpu" class="space-y-4">
      <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
        <h3 class="text-xl font-semibold mb-2">{{ cpu.brand }}</h3>
        <p class="text-sm text-gray-400 mb-4">Fabricante: {{ cpu.vendor }}</p>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <span class="block text-gray-400 text-sm">Núcleos (Físicos)</span>
            <span class="font-medium text-lg">{{ cpu.physical_cores }}</span>
          </div>
          <div>
            <span class="block text-gray-400 text-sm">Núcleos (Lógicos)</span>
            <span class="font-medium text-lg"
              >{{ cpu.logical_cores }} Threads</span
            >
          </div>
          <div>
            <span class="block text-gray-400 text-sm">Frequência Base</span>
            <span class="font-medium text-lg"
              >{{ cpu.base_frequency_mhz }} MHz</span
            >
          </div>
          <div>
            <span class="block text-gray-400 text-sm">Clock Atual (Max)</span>
            <span class="font-medium text-lg"
              >{{
                cpu.current_clock_mhz > 0
                  ? cpu.current_clock_mhz + " MHz"
                  : "N/A"
              }}
              <span class="text-xs text-gray-500">
                / {{ cpu.max_frequency_mhz }} MHz</span
              ></span
            >
          </div>
          <div>
            <span class="block text-gray-400 text-sm"
              >Designação de Soquete</span
            >
            <span class="font-medium text-lg">{{
              cpu.socket_designation
            }}</span>
          </div>
          <div>
            <span class="block text-gray-400 text-sm">Voltagem (Vcore)</span>
            <span class="font-medium text-lg">{{
              cpu.voltage_v > 0 ? cpu.voltage_v.toFixed(3) + " V" : "N/A"
            }}</span>
          </div>
          <div>
            <span class="block text-gray-400 text-sm">Status (Throttling)</span>
            <span class="font-medium text-lg inline-flex items-center gap-2">
              <span v-if="cpu.is_throttling" class="text-yellow-500"
                >Abaixo do Base</span
              >
              <span v-else class="text-green-500">Normal / Boost</span>
            </span>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
          <h3
            class="font-semibold text-lg mb-2 border-b border-gray-800/35 pb-2"
          >
            Cache
          </h3>
          <ul class="space-y-1">
            <li>
              <strong>L1 Cache:</strong>
              {{ cpu.cache_l1_kb > 0 ? cpu.cache_l1_kb + " KB" : "N/A" }}
            </li>
            <li>
              <strong>L2 Cache:</strong>
              {{ cpu.cache_l2_kb > 0 ? cpu.cache_l2_kb + " KB" : "N/A" }}
            </li>
            <li>
              <strong>L3 Cache:</strong>
              {{
                cpu.cache_l3_kb > 0
                  ? (cpu.cache_l3_kb / 1024).toFixed(1) + " MB"
                  : "N/A"
              }}
            </li>
          </ul>
        </div>

        <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
          <h3
            class="font-semibold text-lg mb-2 border-b border-gray-800/35 pb-2"
          >
            Detalhes da Arquitetura
          </h3>
          <p class="text-sm mb-2">{{ cpu.family }}</p>
          <div class="flex flex-wrap gap-1 mt-2">
            <span
              v-for="feat in cpu.features"
              :key="feat"
              class="bg-gray-700 px-2 py-0.5 rounded text-xs"
            >
              {{ feat }}
            </span>
          </div>
        </div>
      </div>

      <!-- New sections below -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Virtualização & Segurança -->
        <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
          <h3
            class="font-semibold text-lg mb-2 border-b border-gray-800/35 pb-2"
          >
            Topologia, Virtualização e Segurança
          </h3>
          <ul class="space-y-1 text-sm text-gray-300">
            <li>
              <strong>Hyper-Threading (HTT):</strong>
              <span :class="cpu.has_htt ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_htt ? "Sim" : "Não"
              }}</span>
            </li>
            <li>
              <strong>Max Logical Proc IDs:</strong>
              {{ cpu.max_logical_processor_ids }}
            </li>
            <li>
              <strong>Intel VT-x (VMX):</strong>
              <span :class="cpu.has_vmx ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_vmx ? "Ativo" : "Indisponível"
              }}</span>
            </li>
            <li>
              <strong>AMD-V (SVM):</strong>
              <span :class="cpu.has_svm ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_svm ? "Ativo" : "Indisponível"
              }}</span>
            </li>
            <li>
              <strong>Modo 64-bit (x64):</strong>
              <span
                :class="cpu.has_64bit_mode ? 'text-green-400' : 'text-gray-500'"
                >{{ cpu.has_64bit_mode ? "Sim" : "Não" }}</span
              >
            </li>
            <li>
              <strong>Segurança NX/XD (Execute Disable):</strong>
              <span
                :class="
                  cpu.has_execute_disable ? 'text-green-400' : 'text-gray-500'
                "
                >{{ cpu.has_execute_disable ? "Sim" : "Não" }}</span
              >
            </li>
          </ul>
        </div>

        <!-- Thermal & Energia -->
        <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
          <h3
            class="font-semibold text-lg mb-2 border-b border-gray-800/35 pb-2"
          >
            Power Management & Performance Monitoring
          </h3>
          <ul class="space-y-1 text-sm text-gray-300">
            <li>
              <strong>Sensor Digital (DTS):</strong>
              <span :class="cpu.has_dts ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_dts ? "Sim" : "Não"
              }}</span>
            </li>
            <li>
              <strong>Limiares de Thermal (Thresholds):</strong>
              {{ cpu.thermal_thresholds }}
            </li>
            <li>
              <strong>Hardware P-States (HWP):</strong>
              <span :class="cpu.has_hwp ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_hwp ? "Suportado" : "Não"
              }}</span>
            </li>
            <li>
              <strong>Hardware Duty Cycling (HDC):</strong>
              <span :class="cpu.has_hdc ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_hdc ? "Suportado" : "Não"
              }}</span>
            </li>
            <li>
              <strong>Notificação de Limite de Força (PLN):</strong>
              <span :class="cpu.has_pln ? 'text-green-400' : 'text-gray-500'">{{
                cpu.has_pln ? "Sim" : "Não"
              }}</span>
            </li>
            <li>
              <strong>PMU (Performance Unit):</strong> Versão
              {{ cpu.pmu_version }} ({{ cpu.pmu_counters }} contadores de
              {{ cpu.pmu_counter_width }} bits)
            </li>
          </ul>
        </div>
      </div>
    </div>
    <p v-else>Carregando CPU...</p>
  </div>
</template>
