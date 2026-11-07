use serde::{Deserialize, Serialize};

/// Identidad criptográfica y credenciales del emisor vía SO_PEERCRED y SO_PEERSEC (Sección 10.4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCredentials {
    pub uid: u32,
    pub gid: u32,
    pub pid: i32,
    pub selinux_context: String,
}

/// Mensajes admitidos en el protocolo seguro de control y ventana de kernel.root (Sección 3.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload", rename_all = "snake_case")]
pub enum IpcMessage {
    /// (1) Lanzar proceso aislado a partir de manifiesto
    LaunchProcess {
        app_id: String,
        manifest_path: String,
    },
    /// (2) Reiniciar, aislar o congelar servicio
    ServiceControl {
        service_id: String,
        operation: ServiceOperation,
    },
    /// (3) Montar/desmontar imagen EROFS verificada
    MountAppImage {
        app_id: String,
        image_path: String,
        mount_point: String,
    },
    UnmountAppImage {
        app_id: String,
    },
    /// (4) Consultar estado de servicio
    QueryServiceStatus {
        service_id: String,
    },
    /// (5) Control de energía (apagar, reiniciar, suspender)
    PowerControl {
        state: PowerState,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceOperation {
    Restart,
    Isolate,
    Freeze,
    Thaw,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerState {
    PowerOff,
    Reboot,
    Suspend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<i32>,
}
