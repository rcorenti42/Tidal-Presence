[![Discord IPC](https://img.shields.io/badge/Discord-IPC-5865F2?logo=discord&logoColor=white)](https://discord.com)
[![WebSocket](https://img.shields.io/badge/WebSocket-real--time-blue)](https://developer.mozilla.org/fr/docs/Web/API/WebSockets_API)
[![Rust](https://img.shields.io/badge/Rust-backend-orange?logo=rust&logoColor=white)](https://rust-lang.org)
[![Windows](https://img.shields.io/badge/Platform-Windows-0078D6?logo=windows&logoColor=white)](https://www.microsoft.com/windows)

# TIDAL Presence

---

Lightweight **Rust application** that displays the currently playing **TIDAL track in Discord Rich Presence**.

Playback data can come from:
- **Local TIDAL desktop playback**
- **Android devices via WebSocket**

This allows Discord to show your currently playing track from **TIDAL Windows or Android**.

---

## Features
- Discord Rich Presence integration
- WebSocket server for external devices
- Compatible with **TIDAL Windows**
- Compatible with **TIDAL Android**
- Real-time playback updates
- Extremely lightweight

---

## Architecture

                  ┌─────────────────────┐
                  │   TIDAL Android     │
                  └─────────┬───────────┘
                            │
                            │ WebSocket
                            ▼
                   ┌───────────────────┐
                   │Tidal Presence App │
                   └─────────┬─────────┘
                             │
                             ▼
                       Discord IPC
                             │
                             ▼
                     Discord Rich Presence

---

## Supported Sources

### Windows Local Desktop

The application can read playback data directly from the **Windows local TIDAL desktop session**.

### Android Device

An Android device running **TIDAL Bridge Android** *(https://github.com/rcorenti42/Tidal-Bridge-Android)* can stream playback data via WebSocket.

---

## Discord Setup

You must create a **Discord Developer Application**.

Steps:

1. Go to the Discord Developer Portal.
2. Create a new application (recommended name: **TIDAL**).
3. Copy the **Application ID**.

Replace the ID in the source code:

       const APP_ID: &str = "APP_ID";

---

## WebSocket Server

The application exposes a WebSocket server used by Android devices.

Exemple:

    ws://192.168.1.42:3000

The Android companion app connects to this endpoint to stream playback metadata.

---

## Build

Install Rust:

    https://rustup.rs

Build:

    cargo build --release

Run:

    cargo run

---

## Network Usage

For remote connections (mobile data / external networks), a VPN such as WireGuard is recommended.

Exemple:

    Phone (5G)
       │
    WireGuard
       │
    Home Network
       │
    Presence Server

---

## Disclaimer

This project is **not affiliated with TIDAL or Discord**.
It is an independent project created to enable Rich Presence support for TIDAL.

---
---
