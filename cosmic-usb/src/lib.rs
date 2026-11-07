#![deny(warnings)]

//! # cosmic-usb
//!
//! Biblioteca de transporte USB para comunicación con dispositivos CosmicOS.

use thiserror::Error;

/// Errores del transporte USB.
#[derive(Debug, Error)]
pub enum UsbError {
    /// No se encontró ningún dispositivo.
    #[error("No se encontró ningún dispositivo CosmicOS")]
    NoDeviceFound,
    /// Error de conexión con el dispositivo.
    #[error("Error de conexión: {0}")]
    ConnectionError(String),
}

/// Información de un dispositivo CosmicOS conectado.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Número de serie del dispositivo.
    pub serial: String,
    /// Modelo del dispositivo.
    pub model: String,
    /// Estado de conexión.
    pub state: String,
}

/// Transporte USB para comunicación con dispositivos CosmicOS.
pub struct UsbTransport;

impl UsbTransport {
    /// Crea una nueva instancia de transporte USB.
    pub fn new() -> Self {
        Self
    }

    /// Escanea los dispositivos CosmicOS conectados por USB.
    pub fn scan_devices(&self) -> Result<Vec<DeviceInfo>, UsbError> {
        log::debug!("Escaneando dispositivos USB CosmicOS...");
        // Stub: no hay dispositivos reales conectados
        Ok(Vec::new())
    }
}

impl Default for UsbTransport {
    fn default() -> Self {
        Self::new()
    }
}
