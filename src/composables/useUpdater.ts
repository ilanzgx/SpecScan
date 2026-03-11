import { ref, onMounted } from "vue";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export function useUpdater() {
  const isUpdateChecking = ref(false);
  const updateAvailable = ref<null | Awaited<ReturnType<typeof check>>>(null);
  const isUpdating = ref(false);

  const checkForUpdates = async () => {
    try {
      isUpdateChecking.value = true;
      const update = await check();
      if (update?.available) {
        updateAvailable.value = update;
      }
    } catch (error) {
      console.error("Failed to check for updates:", error);
    } finally {
      isUpdateChecking.value = false;
    }
  };

  const installUpdate = async () => {
    if (!updateAvailable.value || isUpdating.value) return;
    try {
      isUpdating.value = true;
      await updateAvailable.value.downloadAndInstall();
      await relaunch();
    } catch (error) {
      console.error("Failed to install update:", error);
      isUpdating.value = false;
    }
  };

  onMounted(() => {
    checkForUpdates();
  });

  return {
    isUpdateChecking,
    updateAvailable,
    isUpdating,
    checkForUpdates,
    installUpdate,
  };
}
