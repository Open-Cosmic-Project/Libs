use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceState {
    Starting,
    Running,
    Hanging,
    Failed,
    Isolated,
    Stopped,
}

/// Política de reinicio y supervisión con backoff exponencial (Sección 3.1)
/// Esperas de 1, 2, 4, 8, 16s entre reinicios y aislamiento al 5to fallo en ventana de 10 min.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRestartPolicy {
    pub max_consecutive_failures: u32,
    pub failure_window_seconds: u64,
    pub backoff_delays_seconds: Vec<u64>,
}

impl Default for ServiceRestartPolicy {
    fn default() -> Self {
        Self {
            max_consecutive_failures: 5,
            failure_window_seconds: 600,
            backoff_delays_seconds: vec![1, 2, 4, 8, 16],
        }
    }
}
