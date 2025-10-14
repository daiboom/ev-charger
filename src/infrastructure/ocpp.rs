//! OCPP (Open Charge Point Protocol) 클라이언트

use crate::prelude::*;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

pub struct OcppClient {
    websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    server_url: String,
    charger_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcppMessage {
    pub message_type: u8,
    pub message_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

impl OcppClient {
    pub async fn new(config: OcppConfig) -> AppResult<Self> {
        let mut client = Self {
            websocket: None,
            server_url: config.server_url,
            charger_id: config.charger_id,
        };
        
        client.connect().await?;
        Ok(client)
    }
    
    async fn connect(&mut self) -> AppResult<()> {
        let url = format!("{}/{}", self.server_url, self.charger_id);
        let (ws_stream, _) = connect_async(&url).await
            .map_err(|e| AppError::ChargingError(format!("OCPP 연결 실패: {}", e)))?;
        
        self.websocket = Some(ws_stream);
        info!("OCPP 서버 연결됨: {}", url);
        Ok(())
    }
    
    pub async fn send_start_transaction(&self, session_id: &str) -> AppResult<()> {
        let message = OcppMessage {
            message_type: 2, // CALL
            message_id: uuid::Uuid::new_v4().to_string(),
            action: "StartTransaction".to_string(),
            payload: serde_json::json!({
                "connectorId": 1,
                "idTag": session_id,
                "meterStart": 0,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        };
        
        self.send_message(message).await
    }
    
    async fn send_message(&self, message: OcppMessage) -> AppResult<()> {
        // WebSocket으로 OCPP 메시지 전송
        let json_msg = serde_json::to_string(&message)?;
        info!("OCPP 메시지 전송: {}", json_msg);
        
        // 실제 WebSocket 전송 코드는 더 복잡하지만 여기서는 간소화
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OcppConfig {
    pub server_url: String,
    pub charger_id: String,
    pub heartbeat_interval: u64,
}
