<script setup lang="ts">
import type { NetworkInfo, NetworkAdapterInfo } from "../../types/hardware";

defineProps<{
  adapters: NetworkAdapterInfo[] | null;
  interfaces: NetworkInfo[] | null;
  formatBytes: (bytes: number) => string;
}>();
</script>

<template>
  <div class="p-4 space-y-6">
    <h2 class="text-2xl font-bold">Rede</h2>

    <div v-if="adapters && adapters.length > 0" class="space-y-4">
      <h3 class="text-xl font-bold">Adaptadores Físicos</h3>
      <div
        v-for="adapter in adapters"
        :key="adapter.mac_address"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
        <h4 class="font-bold border-b border-white/10 pb-1 mb-2">
          {{ adapter.name }}
        </h4>
        <ul class="space-y-1 text-sm">
          <li><strong>Fabricante:</strong> {{ adapter.manufacturer }}</li>
          <li><strong>Tipo de Conexão:</strong> {{ adapter.adapter_type }}</li>
          <li>
            <strong>Velocidade Mapeada:</strong>
            {{
              adapter.speed_mbps ? `${adapter.speed_mbps} Mbps` : "Desconhecida"
            }}
          </li>
          <li><strong>MAC Address:</strong> {{ adapter.mac_address }}</li>
          <li><strong>ID de Conexão:</strong> {{ adapter.connection_id }}</li>
        </ul>
      </div>
    </div>

    <div v-if="interfaces && interfaces.length > 0" class="space-y-4">
      <h3 class="text-xl font-bold">Interfaces de Tráfego Ativas</h3>
      <div
        v-for="net in interfaces"
        :key="net.interface_name"
        class="bg-white/5 backdrop-blur-xl shadow-2xl p-4 rounded-xl"
      >
        <h4 class="font-bold border-b border-white/10 pb-1 mb-2">
          Conexão: {{ net.interface_name }}
        </h4>
        <ul class="space-y-1 text-sm">
          <li><strong>MAC Address:</strong> {{ net.mac_address }}</li>
          <li>
            <strong>Dados Recebidos (Download):</strong>
            {{ formatBytes(net.received_bytes) }}
          </li>
          <li>
            <strong>Dados Enviados (Upload):</strong>
            {{ formatBytes(net.transmitted_bytes) }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
