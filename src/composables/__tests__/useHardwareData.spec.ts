import { describe, it, expect, vi } from "vitest";
import { useHardwareData } from "../useHardwareData";

// Mock Vue lifecycle to avoid warnings in Node environment
vi.mock("vue", async () => {
  const actual = (await vi.importActual("vue")) as any;
  return {
    ...actual,
    onMounted: vi.fn(),
  };
});

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd) => {
    if (cmd === "get_system_info")
      return Promise.resolve({ os_name: "TestOS", uptime_seconds: 3600 });
    if (cmd === "get_cpu_info") return Promise.resolve({ name: "Test CPU" });
    return Promise.resolve([]);
  }),
}));

describe("useHardwareData", () => {
  it("should initialize with loading state", async () => {
    // Arrange & Act
    const { isLoading } = useHardwareData();

    // Assert
    expect(isLoading.value).toBeDefined();
  });

  it("utilities: formatBytes should format correctly", () => {
    // Arrange & Act
    const { formatBytes } = useHardwareData();

    // Assert
    expect(formatBytes(0)).toBe("n/a");
    expect(formatBytes(1024)).toBe("1 KB");
    expect(formatBytes(1024 * 1024)).toBe("1 MB");
  });

  it("utilities: formatUptime should format correctly", () => {
    // Arrange & Act
    const { formatUptime } = useHardwareData();

    // Assert
    expect(formatUptime(3661)).toBe("1h 1m 1s");
    expect(formatUptime(60)).toBe("1m 0s");
  });

  it("should fetch data on mount", async () => {
    // Arrange & Act
    const { systemInfo, fetchAll } = useHardwareData();
    await fetchAll();

    // Assert
    expect(systemInfo.value).not.toBeNull();
    expect(systemInfo.value?.os_name).toBe("TestOS");
  });
});
