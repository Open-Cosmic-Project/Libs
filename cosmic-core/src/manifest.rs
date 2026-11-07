use serde::{Deserialize, Serialize};

/// Clases de aplicación predeterminadas (Sección 6.3)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppClass {
    Camera,
    Messaging,
    Browser,
    Banking,
    MediaGallery,
    FileManager,
    Settings,
    System,
    Generic,
}

/// Encabezado del manifiesto de aplicación (Sección 6.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestHeader {
    pub name: String,
    pub developer: String,
    pub repository_url: String,
    pub signing_key_id: String,
    pub language: String,
    pub framework: String,
    pub schema_version: u32,
    pub target_abi: String,
}

/// Cuerpo del manifiesto de aplicación (Sección 6.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestBody {
    pub functionality: String,
    pub app_type: String,
    pub app_class: AppClass,
    pub system_dependencies: Vec<String>,
}

/// Cola del manifiesto de aplicación (Sección 6.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestTail {
    pub declared_permissions: Vec<String>,
    pub binary_size_bytes: u64,
    pub origin_hash: String,
    pub hash_algorithm: String,
}

/// Estructura completa del Manifest.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppManifest {
    pub header: ManifestHeader,
    pub body: ManifestBody,
    pub tail: ManifestTail,
}

impl AppManifest {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
