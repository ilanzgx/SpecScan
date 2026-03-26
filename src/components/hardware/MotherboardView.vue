<script setup lang="ts">
import type { MotherboardInfo } from "../../types/hardware";

defineProps<{
  motherboard: MotherboardInfo | null;
}>();

const parseSmbiosValue = (rawString: string | undefined): string => {
  if (!rawString) return "";
  const match = rawString.match(/value:\s*([A-Za-z0-9_-]+)/);
  if (match && match[1] && match[1] !== "Unknown") {
    return match[1];
  }
  return "";
};
</script>

<template>
  <div class="p-4 space-y-6">
    <h2 class="text-2xl font-bold">Placa-Mãe</h2>

    <div v-if="motherboard">
      <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
        <h3 class="text-xl font-bold mb-2">Informações Gerais</h3>
        <ul class="space-y-1">
          <li><strong>Fabricante:</strong> {{ motherboard.manufacturer }}</li>
          <li><strong>Modelo:</strong> {{ motherboard.product }}</li>
          <li><strong>Versão:</strong> {{ motherboard.version }}</li>
          <li>
            <strong>Número de Série:</strong> {{ motherboard.serial_number }}
          </li>
          <li><strong>Asset Tag:</strong> {{ motherboard.asset_tag }}</li>
        </ul>
      </div>

      <div class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl">
        <h3 class="text-xl font-bold mb-2">BIOS / UEFI</h3>
        <ul class="space-y-1">
          <li><strong>Fabricante:</strong> {{ motherboard.bios.vendor }}</li>
          <li><strong>Versão:</strong> {{ motherboard.bios.version }}</li>
          <li>
            <strong>Lançamento:</strong> {{ motherboard.bios.release_date }}
          </li>
        </ul>
      </div>

      <div class="space-y-4">
        <h3 class="text-xl font-bold mb-4">Slots de RAM na Placa</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div
            v-for="slot in motherboard.memory_slots"
            :key="slot.slot_label"
            class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl flex flex-col justify-start"
          >
            <h4 class="font-bold border-b border-white/10 pb-2 mb-3">
              Slot {{ slot.slot_label }} ({{ slot.bank_label }})
            </h4>
            
            <ul v-if="slot.size_mb > 0" class="space-y-2 text-sm">
              <li><strong>Capacidade:</strong> {{ Math.round(slot.size_mb / 1024) }} GB ({{ slot.size_mb }} MB)</li>
              <li>
                <strong>Fabricante e PN:</strong> {{ slot.manufacturer && slot.manufacturer.trim() !== 'Unknown' ? slot.manufacturer : 'Desconhecido' }} —
                {{ slot.part_number && slot.part_number.trim() !== 'Unknown' ? slot.part_number : 'Desconhecido' }}
              </li>
              <li>
                <strong>Tipo de RAM:</strong> 
                <template v-if="parseSmbiosValue(slot.memory_type) || parseSmbiosValue(slot.form_factor)">
                  {{ parseSmbiosValue(slot.memory_type) }} {{ parseSmbiosValue(slot.form_factor) }}
                </template>
                <template v-else>
                  Desconhecido
                </template>
                <template v-if="slot.speed_mhz > 0">
                   @ {{ slot.speed_mhz }} MHz
                </template>
              </li>
              <li><strong>Número de Série:</strong> {{ slot.serial_number && slot.serial_number.trim() !== 'Unknown' ? slot.serial_number : 'Desconhecido' }}</li>
              <li>
                <strong>Tensão Padrão:</strong> 
                {{ slot.configured_voltage && slot.configured_voltage !== 'Unknown' ? slot.configured_voltage : 'Desconhecido' }}
              </li>
            </ul>
            <div v-else class="text-sm text-gray-400 italic mt-2 flex items-center justify-center h-full">
              Slot vazio
            </div>
          </div>
        </div>
      </div>
    </div>
    <p v-else>Carregando placa-mãe...</p>
  </div>
</template>
