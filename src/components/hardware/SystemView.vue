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

import { computed } from "vue";

const formattedMemory = computed(() => {
  if (!props.memorySlots || props.memorySlots.length === 0) {
    return props.memory
      ? props.formatBytes(props.memory.total_memory)
      : "Desconhecido";
  }

  const populatedSlots = props.memorySlots.filter((s) => s.capacity_gb > 0);

  if (populatedSlots.length === 0) {
    return props.memory
      ? props.formatBytes(props.memory.total_memory)
      : "Desconhecido";
  }

  const slotCount = populatedSlots.length;

  const stickSizeGB = populatedSlots[0]?.capacity_gb ?? 0;
  const stickSpeed = populatedSlots[0]?.speed_mhz ?? 0;
  const stickType = populatedSlots[0]?.memory_type ?? "";

  let totalSum = 0;
  for (const slot of props.memorySlots) {
    totalSum += slot.capacity_gb * 1024 * 1024 * 1024;
  }

  return `${props.formatBytes(totalSum)} (${props.formatBytes(props.memory?.total_memory as number)} usável) (${slotCount}x${stickSizeGB}GB ${stickType} ${stickSpeed > 0 ? stickSpeed + "MHz" : ""})`.trim();
});
</script>

<template>
  <div class="p-4">
    <div class="mt-4">
      <h2 class="text-2xl font-bold mb-4">Resumo da configuração</h2>
      <ul>
        <!-- os -->
        <li>
          <strong>Sistema:</strong>
          {{ system?.os_name }}
          {{ system?.os_version }}
          {{ system?.cpu_arch }}
        </li>

        <!-- motherboard -->
        <li>
          <strong>Placa-Mãe:</strong>
          {{ motherboard?.product }} por {{ motherboard?.manufacturer }}
        </li>

        <!-- cpu -->
        <li><strong>CPU:</strong> {{ cpu?.brand }}</li>

        <!-- gpu -->
        <li><strong>GPU:</strong> {{ gpu?.name }}</li>

        <!-- memory slots -->
        <li>
          <div class="flex flex-col">
            <span>
              <strong>Memória:</strong>
              {{ formattedMemory }}
            </span>
            <ul
              v-if="memorySlots && memorySlots.length > 0"
              class="ml-5 mt-1 list-disc text-[13px] text-white/50"
            >
              <template v-for="(slot, index) in memorySlots" :key="index">
                <li v-if="slot.capacity_gb > 0">
                  Slot {{ index + 1 }}: {{ slot.capacity_gb }}GB
                  {{ slot.memory_type }}
                  <template v-if="slot.speed_mhz > 0"
                    >{{ slot.speed_mhz }}MHz</template
                  >
                  <template
                    v-if="
                      slot.manufacturer &&
                      slot.manufacturer.trim() !== 'Unknown'
                    "
                  >
                    ({{ slot.manufacturer.trim() }})
                  </template>
                </li>
              </template>
            </ul>
          </div>
        </li>

        <li>
          <div class="flex flex-col">
            <span>
              <strong>Armazenamento:</strong>
              {{ physicalDisks?.length }} disco(s)
            </span>
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
