<script setup lang="ts">
import type {
  SystemInfo,
  CpuInfo,
  GpuInfo,
  MemoryInfo,
  PhysicalMemorySlot,
  PhysicalDiskInfo,
  MotherboardInfo,
  CpuBenchmark,
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
  cpuBenchmark?: CpuBenchmark | null;
  // network: NetworkInfo[] | null;
  formatBytes: (bytes: number) => string;
  uptime: string;
}>();

import { computed, onMounted, ref } from "vue";
import * as htmlToImage from "html-to-image";
import { save, message } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { getVersion } from "@tauri-apps/api/app";

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

const appVersion = ref("");

onMounted(async () => {
  console.log(props.motherboard?.memory_slots);
  try {
    appVersion.value = await getVersion();
  } catch (error) {
    console.warn("Falha ao obter versão:", error);
  }
});

const summaryRef = ref<HTMLElement | null>(null);
const isGenerating = ref(false);

const downloadSummaryImage = async () => {
  if (!summaryRef.value) return;
  try {
    isGenerating.value = true;

    // small delay to ensure UI updated button state
    await new Promise((resolve) => setTimeout(resolve, 50));

    const dataUrl = await htmlToImage.toPng(summaryRef.value, {
      quality: 1,
      pixelRatio: 2,
      backgroundColor: "#0f172a", // background color
      filter: (node) => {
        if (
          node instanceof HTMLElement &&
          node.dataset.ignoreInImage === "true"
        ) {
          return false;
        }
        return true;
      },
    });

    const defaultFilename = `SpecScan-${new Date().toISOString().slice(0, 10)}.png`;

    // open native dialog to choose where to save
    const selectedPath = await save({
      filters: [{ name: "Imagem", extensions: ["png"] }],
      defaultPath: defaultFilename,
    });

    if (selectedPath) {
      // extract only base64 data from dataUrl
      const base64Data = dataUrl.split(",")[1];
      if (!base64Data) throw new Error("Falha ao processar dados da imagem.");

      // convert base64 data to array of bytes (Uint8Array)
      const binaryString = atob(base64Data);
      const bytes = new Uint8Array(binaryString.length);
      for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
      }

      await writeFile(selectedPath, bytes);
    }
  } catch (error: any) {
    console.error("Erro ao gerar a imagem:", error);
    await message(`Erro ao salvar imagem: ${error.message || error}`, {
      title: "Erro",
      kind: "error",
    });
  } finally {
    isGenerating.value = false;
  }
};
</script>

<template>
  <div class="p-4 flex items-center justify-center min-h-full py-6">
    <div
      ref="summaryRef"
      class="relative bg-white/5 backdrop-blur-xl shadow-2xl px-8 py-6 rounded-xl w-full max-w-2xl"
    >
      <div
        class="flex items-center justify-between mb-6 border-b border-gray-800/35 pb-3"
      >
        <h2 class="text-2xl font-bold">Resumo da configuração</h2>

        <button
          @click="downloadSummaryImage"
          :disabled="isGenerating"
          data-ignore-in-image="true"
          class="flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-blue-300 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="Salvar Resumo como Imagem"
        >
          <svg
            v-if="!isGenerating"
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
          <svg
            v-else
            class="animate-spin w-4 h-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          {{ isGenerating ? "Gerando..." : "Gerar Imagem" }}
        </button>
      </div>
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
            <p>
              {{ cpu?.brand }}
              <span
                v-if="cpuBenchmark?.ranking"
                class="text-xs bg-blue-500/20 text-blue-300 px-2 py-0.5 rounded border border-blue-500/30"
              >
                Ranking: #{{ cpuBenchmark.ranking }}
              </span>
            </p>
            <p v-if="cpuBenchmark?.multi_score" class="text-xs px-4 py-0.5">
              <span class="text-[13px] text-white/50">
                PassMark Score:
                {{ cpuBenchmark.multi_score }} (Multi) /
                {{ cpuBenchmark.single_score }} (Single)
              </span>
            </p>
            <p v-if="cpuBenchmark?.release_date" class="text-xs px-4 py-0.5">
              <span class="text-[13px] text-white/50">
                Data de Lançamento: {{ cpuBenchmark.release_date }} | Socket:
                {{ cpuBenchmark.socket }} | TDP:
                {{ cpuBenchmark.tdp !== "NA" ? cpuBenchmark.tdp + "W" : "N/A" }}
                | Núcleos: {{ cpuBenchmark.cores }}
              </span>
            </p>
          </div>
        </li>

        <!-- gpu -->
        <li>
          <div class="">
            <h1 class="flex items-center gap-2">
              <IconGpu class="w-5 h-5" />
              <span class="font-bold text-blue-300">GPU:</span>
            </h1>
            <p>
              {{ gpu?.name }}
              <span
                v-if="gpu?.benchmark?.ranking"
                class="text-xs bg-cyan-500/20 text-cyan-300 px-2 py-0.5 rounded border border-cyan-500/30 ml-2"
              >
                Ranking: #{{ gpu.benchmark.ranking }}
              </span>
            </p>
            <p v-if="gpu?.benchmark?.g3d_score" class="text-xs px-4 py-0.5">
              <span class="text-[13px] text-white/50">
                PassMark Score:
                {{ gpu.benchmark.g3d_score }} (G3D) /
                {{ gpu.benchmark.g2d_score }} (G2D)
              </span>
            </p>
            <p v-if="gpu?.benchmark?.release_date" class="text-xs px-4 py-0.5">
              <span class="text-[13px] text-white/50">
                Data de Lançamento: {{ gpu.benchmark.release_date }}
              </span>
            </p>
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

      <!-- watermark visible only during image generation -->
      <div
        v-if="isGenerating"
        class="absolute bottom-4 right-6 text-white/30 text-[11px] font-medium flex items-center gap-1.5 opacity-90"
      >
        <span class="text-white">SpecScan</span>
        <span class="text-gray-500">{{
          appVersion ? "v" + appVersion : ""
        }}</span>
      </div>
    </div>
  </div>
</template>
