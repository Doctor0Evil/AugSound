# AI audio engine schemas

AugSound uses five parameter schemas to describe desired soundscapes for various smart‑city contexts, combining symbolic music control with neural audio rendering.[web:5]

Each schema is implemented as a JSON Schema document in `schemas/` and is intended to be consumed by an AI engine that separates musical structure from audio synthesis, consistent with contemporary music‑generation research.[web:5]

## EcoWalkway.AmbienceProfile

Captures environmental and physiological context for shaded desert paths, canals, or tree‑lined walkways:

- `biome_type`: High‑level descriptor such as `desert_shade`, `urban_canal`, or `tree_canopy`.
- `foot_traffic_density`: Float 0–1, describing crowding.
- `target_hr_zone`: Categorical (rest, light_walk, brisk_walk).
- `noise_floor_db`: Numeric estimate of ambient noise.
- `wildlife_masking_budget_db`: Maximum acceptable masking of natural sounds.

Engines should reduce high‑frequency content when wildlife masking budget is low and adjust tempo and rhythmic density to nudge heart rate toward the target zone without violating SPL limits.

## GymZone.MotivationTrackSpec

Describes motivational music for fitness zones:

- `zone_type`: Categorical (cardio, strength, stretching, recovery).
- `session_phase`: Categorical (warmup, peak, cooldown).
- `crowding_level`: Float 0–1, typically occupancy ratio.
- `user_preference_vector`: Style weights for electronic, rock, hiphop, orchestral, ambient.
- `speech_presence`: Boolean indicating live coaching or instruction.

Engines should increase tempo and bass energy for cardio peak phases while reducing midrange congestion when speech is present to preserve intelligibility.

## Transit.PodAmbienceState

Defines ambient sound for transit cabins and EVs:

- `route_phase`: boarding, departure, cruise, arrival.
- `occupancy_ratio`: Float 0–1.
- `time_of_day`: Numeric or categorical representation of day period.
- `incident_alert_level`: none, minor_delay, major_disruption.
- `motion_smoothness`: Float 0–1 from accelerometer data.

Engines should prioritize simple, steady textures during disruptions and rough motion while allowing more subtle layers during smooth cruising.

## Retail.AffectGuidanceCue

Guides affect in retail environments:

- `store_zone`: entrance, browsing, fitting_room, checkout, exit.
- `target_affect`: calm, curious, focused, efficient.
- `queue_length`: Numeric.
- `promo_intensity`: Float 0–1 governing promotional feel.
- `ambient_light_level`: Numeric.

Engines should keep promotional cues under the specified maximum and use calmer, predictable patterns for long queues at checkout.

## Civic.QuietRoomSoundscape

Supports quiet rooms and focus pods:

- `room_type`: library_nook, meditation_room, focus_pod.
- `occupation_count`: Numeric.
- `requested_mode`: silence, near_silence, soft_ambience.
- `hrv_target`: Numeric or band related to heart‑rate variability.
- `external_noise_events`: Rate per minute of external noise.

Engines can use near‑silence modes with very low‑level broadband or tonal beds tuned to mask intermittent noise while staying within conservative SPL and clinical comfort guidelines.[web:9]
