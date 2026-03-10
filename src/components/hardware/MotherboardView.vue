<script setup lang="ts">
import type { MotherboardInfo } from "../../types/hardware";

defineProps<{
  motherboard: MotherboardInfo | null;
}>();
</script>

<template>
  <div class="p-4 space-y-6">
    <h2 class="text-2xl font-bold">Placa-Mãe</h2>

    <div v-if="motherboard">
      <h3 class="text-xl font-bold mb-2">Informações Gerais</h3>
      <ul class="space-y-1 mb-4">
        <li><strong>Fabricante:</strong> {{ motherboard.manufacturer }}</li>
        <li><strong>Modelo:</strong> {{ motherboard.product }}</li>
        <li><strong>Versão:</strong> {{ motherboard.version }}</li>
        <li>
          <strong>Número de Série:</strong> {{ motherboard.serial_number }}
        </li>
        <li><strong>Asset Tag:</strong> {{ motherboard.asset_tag }}</li>
      </ul>

      <h3 class="text-xl font-bold mb-2">BIOS / UEFI</h3>
      <ul class="space-y-1 mb-4">
        <li><strong>Fabricante:</strong> {{ motherboard.bios.vendor }}</li>
        <li><strong>Versão:</strong> {{ motherboard.bios.version }}</li>
        <li>
          <strong>Lançamento:</strong> {{ motherboard.bios.release_date }}
        </li>
      </ul>

      <h3 class="text-xl font-bold mb-2">Slots de RAM na Placa</h3>
      <div
        v-for="slot in motherboard.memory_slots"
        :key="slot.slot_label"
        class="mb-4"
      >
        <h4 class="font-bold border-b border-white/10 pb-1 mb-2">
          Slot {{ slot.slot_label }} ({{ slot.bank_label }})
        </h4>
        <ul class="space-y-1">
          <li><strong>Capacidade:</strong> {{ slot.size_mb }} MB</li>
          <li>
            <strong>Fabricante e PN:</strong> {{ slot.manufacturer }} —
            {{ slot.part_number }}
          </li>
          <li>
            <strong>Tipo de RAM:</strong> {{ slot.memory_type }}
            {{ slot.form_factor }} @ {{ slot.speed_mhz }} MHz
          </li>
          <li><strong>Número de Série:</strong> {{ slot.serial_number }}</li>
          <li><strong>Tensão Padrão:</strong> {{ slot.configured_voltage }}</li>
        </ul>
      </div>
    </div>
    <p v-else>Carregando placa-mãe...</p>
  </div>
</template>
