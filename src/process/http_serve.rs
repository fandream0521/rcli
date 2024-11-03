use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServerState {
    pub dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {} on http://{}", dir.display(), addr);

    let state = HttpServerState { dir: dir.clone() };

    let router = Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    Path(path): Path<String>,
    State(state): State<Arc<HttpServerState>>,
) -> (StatusCode, String) {
    let path = state.dir.join(path.trim_start_matches('/'));
    info!("Requesting file: {}", path.display());
    if path.exists() {
        let content = fs::read_to_string(path).await;
        match content {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Failed to read file: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
            }
        }
    } else {
        warn!("File not found: {}", path.display());
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found!", path.display()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = HttpServerState {
            dir: PathBuf::from("."),
        };
        let path = Path("Cargo.toml".to_string());
        let (status, content) = file_handler(path, State(Arc::new(state))).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
