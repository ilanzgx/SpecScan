# SpecScan

A beautiful, elegant and lightweight desktop application to view detailed hardware information.

Built with **Tauri**, **Rust**, **Vue 3**, **TypeScript**, and **TailwindCSS**.

## 🚀 Getting Started

1. Install dependencies:

```bash
npm install
```

2. Run in development mode:

```bash
npm run tauri dev
```

## ✨ Features

- **Deep CPU Info**: Extracts architecture, cache, thermal and virtualization data directly from the CPUID registers.
- **Memory & Slots**: Identifies capacity, speeds, and populated DIMM slots.
- **GPU & Storage**: Scans available graphics cards and physical disk drives.
- **Glassmorphism UI**: A gorgeous translucent dark-mode dashboard.

## 🛠️ Stack

- [Tauri](https://tauri.app/) (Backend / Engine)
- [Rust](https://www.rust-lang.org/) (Hardware calls via `raw-cpuid` and `sysinfo`)
- [Vue 3](https://vuejs.org/) + [Vite](https://vitejs.dev/) (Frontend)
- [Tailwind CSS](https://tailwindcss.com/) (Styling)
