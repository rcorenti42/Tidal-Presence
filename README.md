# TIDAL Presence

---

A lightwight Rust application that creates a **Discord Rich Presence for TIDAL**.

The application can receive track data from:
- **Local TIDAL desktop playback**
- **Android TIDAL playback via WebSocket**

This allows Discord to display the currently playing track from **TIDAL Windows or Android**.

---

## Features
- Discord Rich Presence integration
- WebSocket server for external devices
- Compatible with TIDAL Windows
- Compatible with TIDAL Android
- Real-time playback updates
- Very low resource usage

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

You must create a Discord Developer Application.

Steps:

1. Go to the Discord Developer Portal.
2. Create a new application (recommended name: **TIDAL**).
3. Copy the **Application ID**.
4. Replace it in the source code:

       const APP_ID: &str = "APP_ID";

---

## WebSocket Server

The application exposes a WebSocket server.

Exemple:

    ws://192.168.1.42:3000

Android devices connect to this server to send playback metadata.

---

## Network Usage

For remote usage (mobile data / external networks), a VPN such as WireGuard is recommended.

Exemple:

    Phone (5G)
       │
    WireGuard
       │
    Home Network
       │
    Presence Server

---

## Build

Install Rust:

    https://rustup.rs

Build the project:

    cargo build --release

Run:

    cargo run

---

## License

Personal project. Use freely.

---

