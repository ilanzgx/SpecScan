export interface SystemInfo {
  os_name: string;
  os_version: string;
  kernel_version: string;
  host_name: string;
  cpu_arch: string;
  uptime_seconds: number;
  boot_time_seconds: number;
}

export interface CpuInfo {
  name: string;
  vendor: string;
  brand: string;
  physical_cores: number;
  logical_cores: number;
  base_frequency_mhz: number;
  max_frequency_mhz: number;
  cache_l1_kb: number;
  cache_l2_kb: number;
  cache_l3_kb: number;
  family: string;
  features: string[];
  socket_designation: string;
  voltage_v: number;
  is_throttling: boolean;
  current_clock_mhz: number;

  has_htt: boolean;
  max_logical_processor_ids: number;

  has_vmx: boolean;
  has_svm: boolean;

  has_64bit_mode: boolean;
  has_execute_disable: boolean;
  has_1gib_pages: boolean;
  has_rdtscp: boolean;
  has_invariant_tsc: boolean;

  pmu_version: number;
  pmu_counters: number;
  pmu_counter_width: number;

  has_dts: boolean;
  has_turbo_boost: boolean;
  has_hwp: boolean;
  has_hdc: boolean;
  thermal_thresholds: number;
  has_pln: boolean;
}

export interface MemoryInfo {
  total_memory: number;
  used_memory: number;
  free_memory: number;
}

export interface BiosInfo {
  vendor: string;
  version: string;
  release_date: string;
}

export interface MemorySlot {
  slot_label: string;
  bank_label: string;
  manufacturer: string;
  serial_number: string;
  part_number: string;
  size_mb: number;
  speed_mhz: number;
  memory_type: string;
  form_factor: string;
  configured_voltage: string;
}

export interface MotherboardInfo {
  manufacturer: string;
  product: string;
  version: string;
  serial_number: string;
  asset_tag: string;
  bios: BiosInfo;
  memory_slots: MemorySlot[];
}

export interface DiskInfo {
  name: string;
  mount_point: string;
  kind: string;
  file_system: string;
  total_gb: number;
  used_gb: number;
  available_gb: number;
  usage_percent: number;
  is_removable: boolean;
}

export interface PhysicalDiskInfo {
  model: string;
  serial_number: string;
  size_gb: number;
}

export interface PhysicalMemorySlot {
  manufacturer: string;
  part_number: string;
  capacity_gb: number;
  speed_mhz: number;
  configured_voltage: number;
  memory_type: string;
  form_factor: string;
}

export interface GpuInfo {
  name: string;
  manufacturer: string;
  video_processor: string;
  driver_version: string;
  driver_date: string;
  adapter_ram_gb: number;
  adapter_dac_type: string;
  resolution: string;
  bits_per_pixel: number;
  refresh_rate: number;
  max_refresh_rate: number;
  min_refresh_rate: number;
  video_mode_description: string;
}

export interface NetworkInfo {
  interface_name: string;
  received_bytes: number;
  transmitted_bytes: number;
  mac_address: string;
}

export interface NetworkAdapterInfo {
  name: string;
  manufacturer: string;
  mac_address: string;
  speed_mbps: number;
  connection_id: string;
  adapter_type: string;
}
