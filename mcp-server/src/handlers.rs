//! HTTP handlers

use axum::{
    body::Bytes,
    extract::State,
    http::HeaderMap,
    Json,
};
use chrono::Utc;

use crate::{
    models::{ProcessedEvent, WebhookHeaders, WebhookPayload},
    validate_webhook_signature, AppState, ServerError,
};

/// Health check endpoint
pub async fn health_check() -> Result<Json<serde_json::Value>, ServerError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

/// GitHub webhook endpoint
pub async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<serde_json::Value>, ServerError> {
    // Extract headers
    let webhook_headers = extract_webhook_headers(&headers)?;

    // Validate signature if secret is configured
    if !state.config.github_webhook_secret.is_empty() {
        if let Some(signature) = &webhook_headers.signature {
            validate_webhook_signature(
                &state.config.github_webhook_secret,
                signature,
                &body,
            )?;
        } else {
            return Err(ServerError::Validation(
                "Missing GitHub signature header".to_string(),
            ));
        }
    }

    // Parse payload
    let payload: WebhookPayload = serde_json::from_slice(&body)
        .map_err(|e| ServerError::Validation(e.to_string()))?;

    // Process the event
    let processed_event = process_webhook_event(webhook_headers.event, payload)?;

    // Store in history
    if state.config.max_event_history > 0 {
        let mut history = state.event_history.lock().await;
        history.push(serde_json::to_string(&processed_event).unwrap());
        if history.len() > state.config.max_event_history {
            history.remove(0);
        }
    }

    Ok(Json(serde_json::json!({
        "status": "processed",
        "event": processed_event.event_type,
        "repository": processed_event.repository,
        "timestamp": processed_event.timestamp
    })))
}

/// List recent events
pub async fn list_events(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ServerError> {
    let history = state.event_history.lock().await;
    
    let events: Result<Vec<ProcessedEvent>, _> = history
        .iter()
        .map(|event_str| serde_json::from_str(event_str))
        .collect();

    match events {
        Ok(events) => Ok(Json(serde_json::json!({
            "count": events.len(),
            "events": events
        }))),
        Err(e) => Err(ServerError::Runtime(e.to_string())),
    }
}

/// Extract webhook headers
fn extract_webhook_headers(headers: &HeaderMap) -> Result<WebhookHeaders, ServerError> {
    let event = headers
        .get("X-GitHub-Event")
        .ok_or_else(|| {
            ServerError::Validation("Missing X-GitHub-Event header".to_string())
        })?
        .to_str()
        .map_err(|e| ServerError::Validation(e.to_string()))?
        .to_string();

    let delivery = headers
        .get("X-GitHub-Delivery")
        .ok_or_else(|| {
            ServerError::Validation("Missing X-GitHub-Delivery header".to_string())
        })?
        .to_str()
        .map_err(|e| ServerError::Validation(e.to_string()))?
        .to_string();

    let signature = headers
        .get("X-Hub-Signature-256")
        .map(|v| v.to_str().map(|s| s.to_string()))
        .transpose()
        .map_err(|e| ServerError::Validation(e.to_string()))?;

    Ok(WebhookHeaders {
        event,
        delivery,
        signature,
    })
}

/// Process webhook event
fn process_webhook_event(
    event_type: String,
    payload: WebhookPayload,
) -> Result<ProcessedEvent, ServerError> {
    let repository_name = payload
        .repository
        .as_ref()
        .map(|r| r.full_name.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let sender_name = payload
        .sender
        .as_ref()
        .map(|s| s.login.clone())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(ProcessedEvent {
        event_type,
        repository: repository_name,
        action: payload.action,
        sender: sender_name,
        timestamp: Utc::now().to_rfc3339(),
        payload: serde_json::to_value(payload.event_data).unwrap(),
    })
}
