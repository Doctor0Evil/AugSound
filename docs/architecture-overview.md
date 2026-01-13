# Architecture overview

AugSound treats smart‑city audio as a layered system that reuses existing physical infrastructure and adds semantic control APIs for AI ambience engines.[web:1][web:4][web:9]

## Physical infrastructure patterns

- Smart glazing with audio rails: Electrochromic smart windows used at Phoenix Sky Harbor can host low‑profile powered audio rails and in‑frame transducers to turn glass into a distributed speaker surface.[web:1][web:4]
- Hearing loop wellness layer: Existing induction loops that connect hearing aids to PA channels can be extended with a second, low‑level wellness bus driven by an AI ambience engine, while preserving priority for safety announcements.[web:1]
- In‑ceiling and in‑wall grids: Commodity multizone ceiling speaker systems, as sold for wellness centers and lobbies, become the output layer for AI‑driven soundscapes once the fixed playlist source is replaced by a networked AI node.[web:9][web:16]

XR devices and vehicle cabins complement building audio by offering per‑person channels while ambient building audio remains low‑level for inclusivity.[web:11][web:3]

## Control and semantics

Each physical or virtual zone is driven by a compact parameter schema that describes the desired ambience or music in terms of human context, safety constraints, and environmental conditions. These schemas are referred to as “teaching syntaxes” and are designed to be consumed by symbolic and neural audio models.[web:5]

The Rust zone controller in this repository exposes a minimal HTTP API for one of these syntaxes (`GymZone.MotivationTrackSpec`), allowing BMS, IoT dashboards, or XR middleware to update zone state in real time.

## Runtime topology

Typical deployment in a Phoenix smart‑city testbed:

- Edge node in a building or civic lab runs the zone controller and an AI ambience engine.
- Local IoT and BMS provide sensor inputs such as occupancy counts, light levels, temperature, and event schedules.[web:4][web:13]
- The AI engine periodically pulls or subscribes to zone specs from the controller and renders appropriate audio streams to amplifiers, hearing loop drivers, or XR endpoints.

This pattern matches the region’s existing use of smart‑city iLabs and digital equity hubs as testbeds for IoT and AV systems.[web:4][web:15]
