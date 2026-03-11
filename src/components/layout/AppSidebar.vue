<script setup lang="ts">
import { useUpdater } from "../../composables/useUpdater";
import IconSystem from "../icons/IconSystem.vue";
import IconCpu from "../icons/IconCpu.vue";
import IconGpu from "../icons/IconGpu.vue";
import IconMemory from "../icons/IconMemory.vue";
import IconDisk from "../icons/IconDisk.vue";
import IconMotherboard from "../icons/IconMotherboard.vue";
import IconNetwork from "../icons/IconNetwork.vue";

const props = defineProps<{
  activeTab: string;
}>();

const emit = defineEmits<{
  (e: "update:activeTab", value: string): void;
}>();

const tabs = [
  { id: "system", label: "Sistema", icon: IconSystem },
  { id: "cpu", label: "CPU", icon: IconCpu },
  { id: "gpu", label: "GPU", icon: IconGpu },
  { id: "memory", label: "Memória", icon: IconMemory },
  { id: "disks", label: "Discos", icon: IconDisk },
  { id: "motherboard", label: "Placa-Mãe", icon: IconMotherboard },
  { id: "network", label: "Rede", icon: IconNetwork },
] as const;

const appVersion = import.meta.env.VITE_APP_VERSION;

const { isUpdateChecking, updateAvailable, isUpdating, installUpdate } =
  useUpdater();
</script>

<template>
  <aside
    class="w-56 shrink-0 flex flex-col bg-[#0a0a0e]/55 border-r border-white/10 px-4 py-6 backdrop-blur-md"
  >
    <!-- Header / Logo -->
    <div class="flex items-center pb-6 border-b border-white/10 mb-4">
      <h1 class="text-lg font-bold tracking-tight text-white/90 px-1">
        SpecScan
      </h1>
    </div>

    <!-- Navigation Tabs -->
    <nav class="flex flex-col gap-1.5 flex-1 mt-2">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="flex items-center gap-3 w-full px-4 py-3.5 rounded-lg cursor-pointer font-sans text-[15px] font-medium text-left transition-colors duration-150 select-none group"
        :class="
          activeTab === tab.id
            ? 'bg-white/10 text-white shadow-sm'
            : 'text-white/45 hover:bg-white/5 hover:text-white/90'
        "
        @click="emit('update:activeTab', tab.id)"
      >
        <component
          :is="tab.icon"
          class="w-5 h-5 shrink-0 opacity-80 group-hover:opacity-100 transition-opacity"
          :class="activeTab === tab.id ? 'opacity-100' : ''"
        />
        <span class="flex-1">{{ tab.label }}</span>
      </button>
    </nav>

    <!-- Footer & Updater -->
    <div class="flex flex-col gap-3 pt-4 border-t border-white/10 mt-4">
      <!-- Update Status -->
      <div v-if="updateAvailable" class="flex flex-col gap-2">
        <button
          @click="installUpdate"
          :disabled="isUpdating"
          class="flex items-center justify-center gap-2 w-full py-2 bg-blue-500/20 hover:bg-blue-500/30 text-blue-400 rounded-md text-xs font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span
            v-if="isUpdating"
            class="w-3 h-3 border-2 border-blue-400 border-t-transparent rounded-full animate-spin"
          ></span>
          <svg
            v-else
            xmlns="http://www.w3.org/2000/svg"
            class="w-4 h-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
          {{
            isUpdating
              ? "Instalando..."
              : "Nova versão (" + updateAvailable.version + ")"
          }}
        </button>
      </div>
      <div
        v-else-if="isUpdateChecking"
        class="flex items-center justify-center gap-2"
      >
        <span
          class="w-3 h-3 border-2 border-white/30 border-t-white/80 rounded-full animate-spin"
        ></span>
        <span class="text-[11px] text-white/40">Procurando updates...</span>
      </div>

      <!-- App Version -->
      <div class="flex items-center justify-center">
        <span class="text-[12px] font-medium text-white/30 tracking-wide"
          >v{{ appVersion }}
        </span>
      </div>
    </div>
  </aside>
</template>
