<script setup lang="ts">
const props = defineProps<{
  activeTab: string;
}>();

const emit = defineEmits<{
  (e: "update:activeTab", value: string): void;
}>();

const tabs = [
  { id: "system", label: "Sistema", icon: "🖥️" },
  { id: "cpu", label: "CPU", icon: "⚡" },
  { id: "gpu", label: "GPU", icon: "🎮" },
  { id: "memory", label: "Memória", icon: "🧠" },
  { id: "disks", label: "Discos", icon: "💾" },
  { id: "motherboard", label: "Placa-Mãe", icon: "🔌" },
  { id: "network", label: "Rede", icon: "🌐" },
] as const;
</script>

<template>
  <aside
    class="w-56 shrink-0 flex flex-col bg-[#0a0a0e]/55 border-r border-white/10 px-4 py-6 backdrop-blur-md"
  >
    <!-- Header / Logo -->
    <div
      class="flex items-center gap-3 px-2 pb-6 border-b border-white/10 mb-4 bg-red-500"
    >
      <h1 class="text-[17px] font-bold tracking-tight text-white/90">
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
        <span
          class="text-[20px] leading-none opacity-80 group-hover:opacity-100 transition-opacity"
          :class="activeTab === tab.id ? 'opacity-100' : ''"
          >{{ tab.icon }}</span
        >
        <span class="flex-1">{{ tab.label }}</span>
      </button>
    </nav>

    <!-- Footer -->
    <div class="pt-4 border-t border-white/10 mt-4">
      <span class="text-[12px] font-medium text-white/30 px-2 tracking-wide"
        >v0.1.0</span
      >
    </div>
  </aside>
</template>
