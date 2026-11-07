//! # cosmic-core
//! Librería compartida para componentes y servicios centrales de CosmicOS.
//! Define contratos, serialización de manifiestos, mensajes IPC seguros y políticas.

pub mod ipc;
pub mod manifest;
pub mod service;

pub use ipc::{IpcMessage, IpcResponse, PeerCredentials};
pub use manifest::{AppClass, AppManifest, ManifestHeader, ManifestBody, ManifestTail};
pub use service::{ServiceState, ServiceRestartPolicy};
