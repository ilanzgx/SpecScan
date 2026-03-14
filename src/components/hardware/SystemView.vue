<script setup lang="ts">
import type {
  SystemInfo,
  CpuInfo,
  GpuInfo,
  MemoryInfo,
  PhysicalMemorySlot,
  PhysicalDiskInfo,
  MotherboardInfo,
  // NetworkInfo,
} from "../../types/hardware";

import {
  IconSystem,
  IconMotherboard,
  IconCpu,
  IconGpu,
  IconMemory,
  IconDisk,
} from "../icons";

const props = defineProps<{
  system: SystemInfo | null;
  cpu: CpuInfo | null;
  gpu: GpuInfo | null;
  memory: MemoryInfo | null;
  memorySlots: PhysicalMemorySlot[] | null;
  physicalDisks: PhysicalDiskInfo[] | null;
  motherboard: MotherboardInfo | null;
  // network: NetworkInfo[] | null;
  formatBytes: (bytes: number) => string;
  uptime: string;
}>();

import { computed, onMounted } from "vue";

const parseMemoryType = (typeRaw: string) => {
  if (!typeRaw) return "";
  const match = typeRaw.match(/value:\s*(\w+)/);
  if (match && match[1] && match[1] !== "Unknown") {
    return match[1];
  }
  return "";
};

const formattedMemory = computed(() => {
  if (
    !props.motherboard?.memory_slots ||
    props.motherboard.memory_slots.length === 0
  ) {
    return props.memory
      ? props.formatBytes(props.memory.total_memory)
      : "Desconhecido";
  }

  const populatedSlots = props.motherboard.memory_slots.filter(
    (s) => s.size_mb > 0,
  );

  if (populatedSlots.length === 0) {
    return props.memory
      ? props.formatBytes(props.memory.total_memory)
      : "Desconhecido";
  }

  const slotCount = populatedSlots.length;

  const stickSizeGB = Math.round((populatedSlots[0]?.size_mb ?? 0) / 1024);
  const stickSpeed = populatedSlots[0]?.speed_mhz ?? 0;
  const stickType = parseMemoryType(populatedSlots[0]?.memory_type ?? "");

  let totalSum = 0;
  for (const slot of props.motherboard.memory_slots) {
    totalSum += slot.size_mb * 1024 * 1024;
  }

  return `${props.formatBytes(totalSum)} (${props.formatBytes(props.memory?.total_memory as number)} utilizável) (${slotCount}x${stickSizeGB}GB ${stickType} ${stickSpeed > 0 ? stickSpeed + "MHz" : ""})`.trim();
});

onMounted(() => {
  console.log(props.motherboard?.memory_slots);
});
</script>

<template>
  <div class="p-4 flex items-center justify-center min-h-full py-12">
    <div
      class="bg-white/5 backdrop-blur-xl shadow-2xl p-8 rounded-xl w-full max-w-2xl"
    >
      <h2 class="text-2xl font-bold mb-6 border-b border-gray-800/35 pb-3">
        Resumo da configuração
      </h2>
      <ul class="space-y-4">
        <!-- os -->
        <li>
          <div class="flex items-center gap-2">
            <IconSystem class="w-5 h-5" />
            <span class="font-bold text-blue-300">Sistema:</span>
          </div>
          {{ system?.os_name }}
          {{ system?.os_version }}
          {{ system?.cpu_arch }}
        </li>

        <!-- motherboard -->
        <li>
          <div class="">
            <h1 class="flex items-center gap-2">
              <IconMotherboard class="w-5 h-5" />
              <span class="font-bold text-blue-300">Placa-Mãe:</span>
            </h1>
            <p>{{ motherboard?.manufacturer }} {{ motherboard?.product }}</p>
          </div>
        </li>

        <!-- cpu -->
        <li>
          <div class="">
            <h1 class="flex items-center gap-2">
              <IconCpu class="w-5 h-5" />
              <span class="font-bold text-blue-300">CPU:</span>
            </h1>
            <p>{{ cpu?.brand }}</p>
          </div>
        </li>

        <!-- gpu -->
        <li>
          <div class="">
            <h1 class="flex items-center gap-2">
              <IconGpu class="w-5 h-5" />
              <span class="font-bold text-blue-300">GPU:</span>
            </h1>
            <p>{{ gpu?.name }}</p>
          </div>
        </li>

        <!-- memory slots -->
        <li>
          <div class="flex flex-col">
            <h1 class="flex items-center gap-2">
              <IconMemory class="w-5 h-5" />
              <span class="font-bold text-blue-300">Memória:</span>
            </h1>
            <p>{{ formattedMemory }}</p>
            <ul
              v-if="
                motherboard?.memory_slots && motherboard.memory_slots.length > 0
              "
              class="ml-5 mt-2 space-y-1.5 text-[13px] text-white/50"
            >
              <li
                v-for="(slot, index) in motherboard.memory_slots"
                :key="index"
              >
                <div class="flex items-center gap-2">
                  <span class="font-medium text-white/70 min-w-16">{{
                    slot.slot_label || `Slot ${index + 1}`
                  }}</span>
                  <span v-if="slot.size_mb === 0" class="text-white/30 italic">
                    (Vazio)
                  </span>
                  <span v-else>
                    {{ Math.round(slot.size_mb / 1024) }}GB
                    {{ parseMemoryType(slot.memory_type) }}
                    <template v-if="slot.speed_mhz > 0">
                      {{ slot.speed_mhz }}MHz
                    </template>
                    <template
                      v-if="
                        slot.manufacturer &&
                        slot.manufacturer.trim() !== 'Unknown'
                      "
                    >
                      <span>({{ slot.manufacturer.trim() }})</span>
                    </template>
                  </span>
                </div>
              </li>
            </ul>
          </div>
        </li>

        <li>
          <div class="flex flex-col">
            <h1 class="flex items-center gap-2">
              <IconDisk class="w-5 h-5" />
              <span class="font-bold text-blue-300">Armazenamento:</span>
            </h1>
            <p>{{ physicalDisks?.length }} disco(s)</p>
            <ul
              v-if="physicalDisks && physicalDisks.length > 0"
              class="ml-5 mt-1 list-disc text-[13px] text-white/50"
            >
              <li v-for="(disk, index) in physicalDisks" :key="index">
                {{ disk.model }} — {{ disk.size_gb.toFixed(1) }}GB
              </li>
            </ul>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>
