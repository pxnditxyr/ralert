pub struct HealthStatusEntity {
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Healthy,
    Degraded,
    Unhealthy,
}
