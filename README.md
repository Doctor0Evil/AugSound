# AugSound

AugSound is a smart‑city audio layer for Phoenix‑class urban regions that embeds adaptive, wellness‑oriented soundscapes into buildings, vehicles, and XR devices without traditional, labor‑intensive speaker and wiring builds.[web:1][web:4] It aligns with existing smart windows, hearing loop accessibility systems, and wellness background music packages already deployed in environments such as Phoenix Sky Harbor and regional health centers.[web:1][web:4][web:9]

AugSound focuses on three main concerns:

- **Physical layer**: Smart glazing, in‑frame exciters, hearing loops, ceiling speakers, and vehicle cabins as distributed audio endpoints.
- Control and semantics: AI “teaching” syntaxes that describe ambience and music requirements per zone.
- Runtime services: A small Rust HTTP controller that accepts structured specs and exposes them to an AI ambience engine.

## Use cases

- Eco‑walkways and shaded canal paths with context‑aware ambient sound.
- Gyms and civic fitness hubs using per‑zone motivational tracks.
- Airport gates and transit cabins with adaptive, non‑intrusive ambiences.
- Civic quiet rooms and library nooks with clinically conservative soundscapes.

## Repository structure

- `docs/` — Architecture and integration notes for Phoenix and similar smart‑city stacks.
- `schemas/` — JSON Schema definitions for five AI “teaching” syntaxes used to drive music/ambience engines.
- `aug_sound_zone_controller/` — A Rust HTTP microservice that manages GymZone specs per audio zone.
- `examples/http_client/` — A minimal TypeScript client showing how smart‑city systems or BMS adapters can call the controller API.

## Getting started

1. Install Rust (1.75+ recommended) and Node.js (18+).
2. Build and run the Rust zone controller:

   ```bash
   cd aug_sound_zone_controller
   cargo run
