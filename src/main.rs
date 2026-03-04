pub mod db;
use crate::db::conn;
use axum::{Json, Router, extract::State, routing::get};
use blake3::Hasher;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;

pub const DEFAULT_SCRIPTS_SHEEBANG: &str = "#!/bin/sh";

pub const SCRIPT: &str = "uvd.sh";

#[derive(Serialize)]
struct SystemStatus {
    status: String,
    version: String,
    message: String,
}
// On définit les licences libres (Open Source) les plus courantes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenSourceLicense {
    MIT,
    Apache2,
    GplV2,
    GplV3,
    AgplV3,
    Bsd2Clause,
    Bsd3Clause,
    Mpl2,
}

// On définit les licences non libres (Propriétaires / Restrictives)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProprietaryLicense {
    Commercial,      // Logiciel payant (B2B)
    Eula,            // Contrat utilisateur final classique (fermé)
    PersonalUseOnly, // Gratuit pour toi, mais interdit en entreprise
    Freeware,        // Gratuit, mais code source fermé
}

// L'énumération principale qui englobe tout ton système
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    Free(OpenSourceLicense),
    NoFree(ProprietaryLicense),
    Custom(String),
}

#[derive(Serialize, Deserialize)]
pub struct Sign {
    developer: String,
    server: String,
}

#[derive(Serialize, Deserialize)]
pub struct Uvd {
    name: String,
    files: Vec<String>,
    description: String,
    repository: String,
    authors: Vec<String>,
    dependencies: HashMap<String, String>,
    signature: Sign,
    hash: HashMap<String, String>,
    version: String,
    arch: Vec<String>,
    script: String,
    license: LicenseType,
}

pub fn calcul_hash(uvd: &mut Uvd) {
    for file in &uvd.files {
        let hash = Hasher::new().update(file.as_bytes()).finalize();
        uvd.hash.insert(file.to_string(), hash.to_string());
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(conn().await);
    let addr = SocketAddr::from(([0, 0, 0, 0], 7789));
    println!("listen on : https://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler(State(_pool): State<PgPool>) -> Json<SystemStatus> {
    let response = SystemStatus {
        status: "online".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        message: "ready to use".to_string(),
    };
    Json(response)
}

#[cfg(test)]
mod test {
    use crate::LicenseType;
    use crate::Sign;
    use crate::Uvd;
    use std::collections::HashMap;

    pub fn get_uvd() -> Uvd {
        let mut deps = HashMap::new();
        deps.insert("blake3".to_string(), "O.O.O".to_string());
        Uvd {
            name: String::from("uvdd"),
            files: vec![
                "README.md".to_string(),
                "src".to_string(),
                "Cargo.toml".to_string(),
                "Cargo.lock".to_string(),
            ],
            description: "Uvd daemon".to_string(),
            repository: "https://github.com/hackia/uvdd".to_string(),
            authors: vec!["Saigo Ekitae".to_string()],
            dependencies: deps,
            signature: Sign {
                developer: String::new(),
                server: String::new(),
            },
            hash: HashMap::new(),
            version: "0.0.0".to_string(),
            arch: vec!["x86_64".to_string()],
            license: LicenseType::Free(crate::OpenSourceLicense::AgplV3),
            script: crate::SCRIPT.to_string(),
        }
    }
    #[test]
    pub fn test_calcul_hash() {
        let mut uvd = get_uvd();
        assert!(uvd.hash.is_empty());
        crate::calcul_hash(&mut uvd);
        assert!(uvd.hash.len() == 4);
    }
    #[test]
    pub fn test_license() {
        let uvd = get_uvd();
        assert_eq!(
            LicenseType::Free(crate::OpenSourceLicense::AgplV3),
            uvd.license
        );
    }
    #[test]
    pub fn test_author() {
        let uvd = get_uvd();
        assert_eq!(uvd.authors.len(), 1);
    }
    #[test]
    pub fn test_scripts() {
        let uvd = get_uvd();
        assert_eq!(crate::SCRIPT, uvd.script);
    }
    #[test]
    pub fn test_version() {
        assert_eq!(get_uvd().version, "0.0.0".to_string());
    }
    #[test]
    pub fn test_arch() {
        assert!(!get_uvd().arch.is_empty());
    }

    #[test]
    pub fn test_name() {
        assert_eq!(get_uvd().name, "uvdd".to_string());
    }
    #[test]
    pub fn test_description() {
        assert!(!get_uvd().description.is_empty());
    }
    #[test]
    pub fn test_dependencies() {
        assert!(!get_uvd().dependencies.is_empty());
    }
    #[test]
    pub fn test_repository() {
        assert!(!get_uvd().repository.is_empty());
    }
}
