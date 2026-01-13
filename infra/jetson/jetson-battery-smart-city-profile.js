const JETSON_PROFILES = {
  nano: {
    model: "jetson_nano_4g",
    aiPerf: { fp16Tflops: 0.5, int8Tops: 0.25 },
    powerModesW: [5, 10],
    typicalUse: [
      "single_camera_kiosk",
      "pilot_people_counting",
      "low_cost_ar_totem"
    ],
    maxYoloStreams: { "1080p": 1, "720p": 2 },
    recommended: false
  },
  tx2: {
    model: "jetson_tx2_nx",
    aiPerf: { fp16Tflops: 1.3, int8Tops: 1.0 },
    powerModesW: [7.5, 15],
    typicalUse: [
      "2_4_camera_bus_stop",
      "elevator_controller",
      "compact_smart_signage"
    ],
    maxYoloStreams: { "1080p": 3, "720p": 4 },
    recommended: false
  },
  xavierNx: {
    model: "jetson_xavier_nx",
    aiPerf: { fp16Tflops: 6.0, int8Tops: 21 },
    powerModesW: [10, 15],
    typicalUse: [
      "gym_gateway",
      "retail_gateway",
      "eco_walkway_node"
    ],
    maxYoloStreams: { "1080p": 8, "720p": 12 },
    recommended: true
  },
  orinNano: {
    model: "jetson_orin_nano_8g",
    aiPerf: { fp16Tflops: 10, int8Tops: 40 },
    powerModesW: [7, 15, 25],
    typicalUse: [
      "solar_pole_node",
      "small_corridor_xr_hub",
      "multi_stream_audio_video"
    ],
    maxYoloStreams: { "1080p": 6, "720p": 10 },
    recommended: true
  },
  orinNx: {
    model: "jetson_orin_nx_16g",
    aiPerf: { fp16Tflops: 20, int8Tops: 100 },
    powerModesW: [10, 20, 25],
    typicalUse: [
      "multi_tenant_gym_gateway",
      "parking_station_hub",
      "transit_station_xr_node"
    ],
    maxYoloStreams: { "1080p": 12, "720p": 16 },
    recommended: true
  },
  agxOrin: {
    model: "jetson_agx_orin_64g",
    aiPerf: { fp16Tflops: 40, int8Tops: 275 },
    powerModesW: [15, 30, 60],
    typicalUse: [
      "mall_hub_cluster",
      "intersection_controller",
      "city_block_edge_cluster"
    ],
    maxYoloStreams: { "1080p": 32, "720p": 48 },
    recommended: false
  }
};

const BATTERY_POLICY = {
  minSocHighLoad: 0.5,
  minSocNormal: 0.3,
  minSocSafeIdle: 0.15,
  yoloStreamScaling: soc => {
    if (soc >= 0.5) return 1.0;
    if (soc >= 0.3) return 0.6;
    if (soc >= 0.15) return 0.3;
    return 0.1;
  },
  ambienceProfile: soc => {
    if (soc >= 0.5) return { pattern: "hi_energy", tempo: 132, density: 0.9 };
    if (soc >= 0.3) return { pattern: "medium_energy", tempo: 112, density: 0.6 };
    if (soc >= 0.15) return { pattern: "low_energy", tempo: 96, density: 0.4 };
    return { pattern: "minimal", tempo: 80, density: 0.2 };
  }
};

function computeCapacity({ family, resolution, desiredStreams, batterySoc }) {
  const profile = JETSON_PROFILES[family];
  if (!profile) throw new Error(`Unknown Jetson family: ${family}`);

  const baseMax = profile.maxYoloStreams[resolution];
  if (!baseMax) throw new Error(`Unsupported resolution: ${resolution}`);

  const scale = BATTERY_POLICY.yoloStreamScaling(batterySoc);
  const allowedStreams = Math.max(1, Math.floor(baseMax * scale));
  const clampedDesired = Math.min(desiredStreams, allowedStreams);

  const ambience = BATTERY_POLICY.ambienceProfile(batterySoc);

  return {
    model: profile.model,
    recommended: profile.recommended,
    baseMaxStreams: baseMax,
    allowedStreams,
    grantedStreams: clampedDesired,
    batterySoc,
    ambience
  };
}

module.exports = {
  JETSON_PROFILES,
  BATTERY_POLICY,
  computeCapacity
};
