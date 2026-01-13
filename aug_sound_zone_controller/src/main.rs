use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ZoneType {
    Cardio,
    Strength,
    Stretching,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SessionPhase {
    Warmup,
    Peak,
    Cooldown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferenceVector {
    /// 0.0–1.0 style weights; these are normalized by the backend AI engine.
    pub electronic: f32,
    pub rock: f32,
    pub hiphop: f32,
    pub orchestral: f32,
    pub ambient: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GymZoneMotivationTrackSpec {
    pub zone_type: ZoneType,
    pub session_phase: SessionPhase,
    /// 0.0–1.0 crowding level, e.g., occupancy_ratio.
    pub crowding_level: f32,
    pub user_preference_vector: UserPreferenceVector,
    /// True if live coaching or other speech is active.
    pub speech_presence: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneState {
    pub id: Uuid,
    pub name: String,
    pub spec: GymZoneMotivationTrackSpec,
    /// Last update timestamp (seconds since Unix epoch).
    pub updated_at: i64,
}

#[derive(Clone, Default)]
pub struct AppState {
    zones: Arc<RwLock<HashMap<Uuid, ZoneState>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateZoneRequest {
    pub name: String,
    pub spec: GymZoneMotivationTrackSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateZoneSpecRequest {
    pub spec: GymZoneMotivationTrackSpec,
}

async fn list_zones(State(state): State<AppState>) -> impl IntoResponse {
    let zones = state.zones.read().await;
    let values: Vec<ZoneState> = zones.values().cloned().collect();
    Json(values)
}

async fn create_zone(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().timestamp();
    let zone_state = ZoneState {
        id,
        name: payload.name,
        spec: payload.spec,
        updated_at: now,
    };

    {
        let mut zones = state.zones.write().await;
        zones.insert(id, zone_state.clone());
    }

    (StatusCode::CREATED, Json(zone_state))
}

async fn get_zone(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let zones = state.zones.read().await;
    if let Some(zone) = zones.get(&id) {
        (StatusCode::OK, Json(zone.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn update_zone_spec(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(payload): Json<UpdateZoneSpecRequest>,
) -> impl IntoResponse {
    let mut zones = state.zones.write().await;
    if let Some(zone) = zones.get_mut(&id) {
        zone.spec = payload.spec;
        zone.updated_at = chrono::Utc::now().timestamp();
        (StatusCode::OK, Json(zone.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn delete_zone(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let mut zones = state.zones.write().await;
    if zones.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    // In a production Phoenix deployment, this service would run on an edge node
    // within a gym, mall, or civic facility and be integrated with the building
    // management system and AI audio backend.
    let state = AppState::default();

    let app = Router::new()
        .route("/zones", get(list_zones).post(create_zone))
        .route(
            "/zones/:id",
            get(get_zone)
                .put(update_zone_spec)
                .delete(delete_zone),
        )
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse().expect("invalid listen addr");
    println!("AugSound zone controller listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
