import axios from "axios";

const BASE_URL = process.env.AUG_SOUND_BASE_URL || "http://localhost:8080";

async function main() {
  // Create a sample gym zone
  const createResp = await axios.post(`${BASE_URL}/zones`, {
    name: "Downtown Phoenix Cardio",
    spec: {
      zone_type: "cardio",
      session_phase: "peak",
      crowding_level: 0.8,
      user_preference_vector: {
        electronic: 0.7,
        rock: 0.2,
        hiphop: 0.5,
        orchestral: 0.1,
        ambient: 0.3
      },
      speech_presence: false
    }
  });

  console.log("Created zone:", createResp.data);

  const zoneId = createResp.data.id;

  // Update the zone to cooldown with speech present
  const updateResp = await axios.put(`${BASE_URL}/zones/${zoneId}`, {
    spec: {
      zone_type: "cardio",
      session_phase: "cooldown",
      crowding_level: 0.6,
      user_preference_vector: {
        electronic: 0.5,
        rock: 0.1,
        hiphop: 0.3,
        orchestral: 0.4,
        ambient: 0.6
      },
      speech_presence: true
    }
  });

  console.log("Updated zone:", updateResp.data);

  // List all zones
  const listResp = await axios.get(`${BASE_URL}/zones`);
  console.log("All zones:", listResp.data);

  // Delete the created zone
  await axios.delete(`${BASE_URL}/zones/${zoneId}`);
  console.log("Deleted zone:", zoneId);
}

main().catch((err) => {
  console.error("Client error:", err.response?.data || err.message);
  process.exit(1);
});
