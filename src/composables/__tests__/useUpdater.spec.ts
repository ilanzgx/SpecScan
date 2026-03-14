import { describe, it, expect, vi, beforeEach } from "vitest";
import { useUpdater } from "../useUpdater";

// Mock Vue lifecycle to avoid warnings in Node environment
vi.mock("vue", async () => {
  const actual = (await vi.importActual("vue")) as any;
  return {
    ...actual,
    onMounted: vi.fn(),
  };
});

const { mockCheck, mockDownloadAndInstall } = vi.hoisted(() => ({
  mockCheck: vi.fn(),
  mockDownloadAndInstall: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-updater", () => ({
  check: mockCheck,
}));

vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: vi.fn(),
}));

describe("useUpdater", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize correctly", () => {
    // Arrange & Act
    const { isUpdateChecking, isUpdating, updateAvailable } = useUpdater();

    // Assert
    expect(isUpdateChecking.value).toBe(false);
    expect(isUpdating.value).toBe(false);
    expect(updateAvailable.value).toBe(null);
  });

  it("should check for updates and find one", async () => {
    // Arrange
    mockCheck.mockResolvedValueOnce({
      available: true,
      version: "1.0.0",
      downloadAndInstall: mockDownloadAndInstall,
    });

    // Act
    const { checkForUpdates, updateAvailable } = useUpdater();
    await checkForUpdates();

    // Assert
    expect(updateAvailable.value).not.toBeNull();
    expect(updateAvailable.value?.available).toBe(true);
  });

  it("should handle check for updates error", async () => {
    // Arrange
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    mockCheck.mockRejectedValueOnce(new Error("Network error"));

    // Act
    const { checkForUpdates, isUpdateChecking } = useUpdater();
    await checkForUpdates();

    // Assert
    expect(isUpdateChecking.value).toBe(false);
    expect(consoleSpy).toHaveBeenCalledWith(
      "Failed to check for updates:",
      expect.any(Error),
    );
    consoleSpy.mockRestore();
  });

  it("should install update successfully", async () => {
    // Arrange
    mockCheck.mockResolvedValueOnce({
      available: true,
      version: "1.0.0",
      downloadAndInstall: mockDownloadAndInstall,
    });

    // Act
    const { checkForUpdates, installUpdate, isUpdating } = useUpdater();
    await checkForUpdates();
    await installUpdate();

    // Assert
    expect(mockDownloadAndInstall).toHaveBeenCalled();
    expect(isUpdating.value).toBe(true);
  });

  it("should handle install update error", async () => {
    // Arrange
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    const mockErrorDownload = vi
      .fn()
      .mockRejectedValueOnce(new Error("Install failed"));
    mockCheck.mockResolvedValueOnce({
      available: true,
      version: "1.0.0",
      downloadAndInstall: mockErrorDownload,
    });

    // Act
    const { checkForUpdates, installUpdate, isUpdating } = useUpdater();
    await checkForUpdates();
    await installUpdate();

    // Assert
    expect(isUpdating.value).toBe(false);
    expect(consoleSpy).toHaveBeenCalledWith(
      "Failed to install update:",
      expect.any(Error),
    );
    consoleSpy.mockRestore();
  });

  it("should not install if no update is available or already updating", async () => {
    // Arrange
    const { installUpdate, isUpdating } = useUpdater();

    // Act
    await installUpdate(); // No update available

    // Assert
    expect(isUpdating.value).toBe(false);

    // Simulate already updating
    isUpdating.value = true;
    await installUpdate();
    expect(isUpdating.value).toBe(true);
  });
});
