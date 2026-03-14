import { ref, onMounted } from "vue";
import { createSharedComposable } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
import type {
  SystemInfo,
  CpuInfo,
  MemoryInfo,
  MotherboardInfo,
  DiskInfo,
  PhysicalDiskInfo,
  PhysicalMemorySlot,
  GpuInfo,
  NetworkInfo,
  NetworkAdapterInfo,
  CpuBenchmark,
  GpuBenchmark,
} from "../types/hardware";

function useHardwareDataInternal() {
  const isLoading = ref(true);
  const systemInfo = ref<SystemInfo | null>(null);
  const cpuInfo = ref<CpuInfo | null>(null);
  const cpuBenchmark = ref<CpuBenchmark | null>(null);
  const memoryInfo = ref<MemoryInfo | null>(null);
  const motherboardInfo = ref<MotherboardInfo | null>(null);
  const disksInfo = ref<DiskInfo[] | null>(null);
  const physicalDisksInfo = ref<PhysicalDiskInfo[] | null>(null);
  const physicalMemoryInfo = ref<PhysicalMemorySlot[] | null>(null);
  const gpuInfo = ref<GpuInfo[] | null>(null);
  const networkInfo = ref<NetworkInfo[] | null>(null);
  const networkAdaptersInfo = ref<NetworkAdapterInfo[] | null>(null);

  async function fetchAll() {
    isLoading.value = true;
    try {
      await Promise.all([
        invoke<SystemInfo>("get_system_info").then(
          (v) => (systemInfo.value = v),
        ),
        invoke<CpuInfo>("get_cpu_info").then(async (v) => {
          cpuInfo.value = v;
          cpuBenchmark.value = await invoke<CpuBenchmark>("get_cpu_benchmark", {
            brand: v.brand,
          });
        }),
        invoke<MemoryInfo>("get_memory_info").then(
          (v) => (memoryInfo.value = v),
        ),
        invoke<MotherboardInfo>("get_motherboard_info").then(
          (v) => (motherboardInfo.value = v),
        ),
        invoke<DiskInfo[]>("get_disks_info").then((v) => (disksInfo.value = v)),
        invoke<PhysicalDiskInfo[]>("get_physical_disks_info").then(
          (v) => (physicalDisksInfo.value = v),
        ),
        invoke<PhysicalMemorySlot[]>("get_physical_memory_info").then(
          (v) => (physicalMemoryInfo.value = v),
        ),
        invoke<GpuInfo[]>("get_gpu_info").then(async (v) => {
          const gpusWithBenchmarks = await Promise.all(
            v.map(async (gpu) => {
              gpu.benchmark = await invoke<GpuBenchmark>("get_gpu_benchmark", {
                name: gpu.name,
              });
              return gpu;
            }),
          );
          gpuInfo.value = gpusWithBenchmarks;
        }),
        invoke<NetworkInfo[]>("get_network_info").then(
          (v) => (networkInfo.value = v),
        ),
        invoke<NetworkAdapterInfo[]>("get_network_adapters_info").then(
          (v) => (networkAdaptersInfo.value = v),
        ),
      ]);
    } finally {
      isLoading.value = false;
    }
  }

  onMounted(fetchAll);

  // kb, mb, gb, tb
  function formatBytes(bytes: number): string {
    const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    if (bytes === 0) return "n/a";
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${Math.round(bytes / Math.pow(1024, i))} ${sizes[i]}`;
  }

  // hh:mm:ss
  function formatUptime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    const parts = [];
    if (h > 0) parts.push(`${h}h`);
    if (m > 0) parts.push(`${m}m`);
    parts.push(`${s}s`);
    return parts.join(" ");
  }

  return {
    isLoading,
    systemInfo,
    cpuInfo,
    cpuBenchmark,
    memoryInfo,
    motherboardInfo,
    disksInfo,
    physicalDisksInfo,
    physicalMemoryInfo,
    gpuInfo,
    networkInfo,
    networkAdaptersInfo,
    formatBytes,
    formatUptime,
    fetchAll,
  };
}

export const useHardwareData = createSharedComposable(useHardwareDataInternal);
