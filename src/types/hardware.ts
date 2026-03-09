export interface SystemInfo {
  os_name: string;
}

export interface CpuInfo {
  name: string;
  cores: number;
  frequency: number;
}

export interface MemoryInfo {
  total_memory: number;
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
