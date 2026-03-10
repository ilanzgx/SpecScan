<script setup lang="ts">
import type { SystemInfo } from "../../types/hardware";

defineProps<{
  system: SystemInfo | null;
  uptime: string;
}>();
</script>

<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold mb-4">Sistema Operacional</h2>
    <ul v-if="system" class="space-y-2">
      <li>
        <strong>Nome / Versão:</strong> {{ system.os_name }}
        {{ system.os_version }}
      </li>
      <li><strong>Kernel:</strong> {{ system.kernel_version }}</li>
      <li><strong>Hostname:</strong> {{ system.host_name }}</li>
      <li><strong>Arquitetura:</strong> {{ system.cpu_arch }}</li>
      <li>
        <strong>Tempo ligado:</strong> {{ uptime }} ({{
          system.uptime_seconds
        }}s)
      </li>
    </ul>
    <p v-else>Carregando sistema...</p>
  </div>
</template>
