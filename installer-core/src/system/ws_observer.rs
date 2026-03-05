//! Websocket Observer — The Bard's Remote Scrying.
//!
//! This module provides a websocket-based observer that allows remote monitoring
//! of the installation progress. It broadcasts `PhaseEvent`s to any connected
//! clients, enabling a "live stream" of the TUI progress on a web-based dashboard.

use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use warp::Filter;
use futures_util::{StreamExt, SinkExt};
use tracing::{info, error};

pub use crate::model::phase::{PhaseEvent, PhaseObserver};

/// A broadcast-enabled observer that relays events to websocket clients.
pub struct WebsocketObserver {
    tx: broadcast::Sender<PhaseEvent>,
    _rt: Arc<Runtime>,
}

impl WebsocketObserver {
    /// Start a websocket server on the specified port and return the observer.
    pub fn new(port: u16) -> Self {
        let (tx, _) = broadcast::channel(100);
        let tx_cloned = tx.clone();
        
        let rt = Arc::new(Runtime::new().expect("failed to create tokio runtime"));
        let rt_cloned = rt.clone();

        thread::spawn(move || {
            rt_cloned.block_on(async {
                let events = warp::any().map(move || tx_cloned.clone());
                
                let ws_route = warp::path("ws")
                    .and(warp::ws())
                    .and(events)
                    .map(|ws: warp::ws::Ws, tx: broadcast::Sender<PhaseEvent>| {
                        ws.on_upgrade(move |socket| handle_client(socket, tx))
                    });

                info!("Websocket Scrying Server started on port {}", port);
                warp::serve(ws_route).run(([0, 0, 0, 0], port)).await;
            });
        });

        Self { tx, _rt: rt }
    }
}

impl PhaseObserver for WebsocketObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        // Broadcast the event to all connected clients
        let _ = self.tx.send(event);
    }
}

async fn handle_client(ws: warp::ws::WebSocket, tx: broadcast::Sender<PhaseEvent>) {
    let (mut ws_tx, _ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        let msg = match serde_json::to_string(&event) {
            Ok(json) => warp::ws::Message::text(json),
            Err(e) => {
                error!("failed to serialize phase event: {}", e);
                continue;
            }
        };

        if let Err(e) = ws_tx.send(msg).await {
            error!("websocket send error: {}", e);
            break;
        }
    }
}

/// A composite observer that multiplexes events to multiple observers.
pub struct CompositeObserver {
    observers: Vec<Box<dyn PhaseObserver + Send>>,
}

impl CompositeObserver {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn add<O: PhaseObserver + Send + 'static>(&mut self, observer: O) {
        self.observers.push(Box::new(observer));
    }
}

impl PhaseObserver for CompositeObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        for observer in &mut self.observers {
            observer.on_event(event.clone());
        }
    }
}
