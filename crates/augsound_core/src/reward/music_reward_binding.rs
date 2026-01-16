#[derive(Clone, Debug)]
pub struct MusicRewardBinding {
    /// 0.0–1.0 scalar: estimated engagement of dopaminergic pathways
    /// (inferred from chill‑frequency, liking scores, and prior PET‑fMRI priors).
    pub dopa_engage: f32,
    /// 0.0–1.0 scalar: estimated engagement of µ‑opioid hotspots
    /// (inferred from reported chills, comfort, and loop intensity).
    pub mu_opioid_engage: f32,
    /// 0.0–1.0 scalar: Cochrane‑aligned adjunct ceiling; values > 0.0 require
    /// explicit pairing with standard care and ban “replacement” claims.
    pub adjunct_ceiling: f32,
    /// Enforces that loop remains within craving/motivation‑aid bounds.
    pub is_within_adjunct_bounds: bool,
}

impl MusicRewardBinding {
    pub fn new(dopa_engage: f32, mu_opioid_engage: f32) -> Self {
        let d = dopa_engage.clamp(0.0, 1.0);
        let m = mu_opioid_engage.clamp(0.0, 1.0);
        // Upper bound from Cochrane medium‑effect guidance; beyond this,
        // system must *not* increase intensity or allow curative framing.
        let adjunct_ceiling = 0.66_f32;
        let combined = (d + m) * 0.5;
        let is_within_adjunct_bounds = combined <= adjunct_ceiling;
        Self {
            dopa_engage: d,
            mu_opioid_engage: m,
            adjunct_ceiling,
            is_within_adjunct_bounds,
        }
    }
}
