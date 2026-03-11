<script setup lang="ts">
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

    <!-- Footer -->
    <div
      class="flex items-center justify-center pt-4 border-t border-white/10 mt-4"
    >
      <span class="text-[12px] font-medium text-white/30 px-2 tracking-wide"
        >v0.1.0</span
      >
    </div>
  </aside>
</template>
