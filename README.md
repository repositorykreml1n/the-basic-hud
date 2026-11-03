# The Basic HUD

![The Basic HUD](assets/cover-0.4.1.png)

A client-side visual HUD for [Horizon](https://playhorizon.dev). **Move it. Size it. Make it yours.**

Thirteen independent widgets: effects, consumables, durability, totem uses, available player health, item cooldowns, coordinates, FPS, ping, speed, CPS, world clock and mini armour. Arrange them directly over the game, tune each style, or use compact icons with useful values. Includes a customizable Tab list and ten interface languages.

## 0.4.1

[English update notes with Russian, Ukrainian, German, French and Spanish summaries](CHANGELOG.md). From the registry's 0.1.0, this adds the visual editor, telemetry widgets, mini armour, Tab customization, localization and protected Save/Discard flows.

## Install

Download `basic_hud-0.4.1.hnmod` from [Releases](https://github.com/repositorykreml1n/the-basic-hud/releases). Install through Horizon's native-mod manager and build with **Performance / Производительная сборка**. The archive contains Rust source; the launcher compiles it into a native instance.

**Pinned compatibility:** Minecraft26.3, Horizon `68ac6f647e7022b2c696e81863d7cface28c9416`, horizon-mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. Do not assume other Nightlies are supported. Dev startup can overflow the stack on this Windows baseline; use Performance.

## Controls

- Right Shift (rebindable), or Options > Mods > The Basic HUD > Settings > Edit HUD.
- Drag to move; wheel/control buttons to resize; arrows to nudge; snap to align.
- Tab hides editor controls; ? replays the first-visit tutorial.
- Save writes changes. Leaving with edits asks Save / Discard changes / Stay. Enter saves and Esc stays in that dialog.
- Reset requires confirmation and changes the draft until saved.

## Data boundaries

This is visual and client-side. It reads only data the client already has. Missing player health remains unknown. Totem uses outside the client's observation cannot be counted; counts reset on death/leaving. Server-plugin-only cooldowns cannot be read. Single-player ping is labelled Local. Speed/CPS/clock are optional. No FPS gain is promised.

Native mods run with the player's rights and are not sandboxed. Read the source before installing. No Minecraft assets or game binaries are included. This is not an official Horizon, Mojang or Microsoft product.

## Build and reproduce

Run `cargo test --locked --all-targets` for the pure logic tests (67 executions, including reused unit cases). `python tools/generate-i18n.py` validates and regenerates the 150-key catalog. `python tools/package.py` writes the deterministic .hnmod; its SHA-256 must match the release notes. For native checks, use the official [Horizon mod workflow](https://playhorizon.dev/docs/make-a-mod) against the pinned commit in compatibility.json. Native and runtime verification details are in [the release notes](docs/release-0.4.1.md).

MIT. Copyright repositorykreml1n. Cover illustration generated with OpenAI image generation; gameplay UI in the illustration is promotional artwork.
