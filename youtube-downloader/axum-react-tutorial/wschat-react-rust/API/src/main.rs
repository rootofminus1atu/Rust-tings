// dependencies
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use axum_extra::routing::SpaRouter;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use shuttle_secrets::SecretStore;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLock,
};
use tower_http::auth::RequireAuthorizationLayer;

type Users = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;
static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
struct Msg {
    name: String,
    uid: Option<usize>,
    message: String,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf
) -> shuttle_axum::ShuttleAxum {
    // We use Secrets.toml to set the BEARER key, just like in a .env file and call it here
    let secret = secrets.get("BEARER").unwrap_or("Bear".to_string());
   // set up router with Secrets
   let router = router(secret, static_folder);

   Ok(router.into())
}

fn router(secret: String, static_folder: PathBuf) -> Router {
    // initialise the Users k/v store and allow the static files to be served
    let users = Users::default();

    // make an admin route for kicking users
    let admin = Router::new()
        .route("/disconnect/:user_id", get(disconnect_user))
        .layer(RequireAuthorizationLayer::bearer(&secret));

    // return a new router and nest the admin route into the websocket route
     Router::new()
        .route("/ws", get(ws_handler))
        .nest("/admin", admin)
        .layer(Extension(users))
}

// "impl IntoResponse" means we want our function to return a websocket connection
async fn ws_handler(ws: WebSocketUpgrade, Extension(state): Extension<Users>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: Users) {
    // When a new user enters the chat (opens the websocket connection), assign them a user ID
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // By splitting the websocket into a receiver and sender, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Create a new channel for async task management (stored in Users hashmap)
    let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = mpsc::unbounded_channel();

    // If a message has been received, send the message (expect on error)
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            sender.send(msg).await.expect("Error while sending message");
        }
        sender.close().await.unwrap();
    });

    // if there's a message and the message is OK, broadcast it along all available open websocket connections
    while let Some(Ok(result)) = receiver.next().await {
        println!("{:?}", result);
        if let Ok(result) = enrich_result(result, my_id) {
            broadcast_msg(result, &state).await;
        }
    }
}

async fn broadcast_msg(msg: Message, users: &Users) {
// "If let" is basically a simple match statement, which is perfect for this use case
// as we want to only match against one condition.
    if let Message::Text(msg) = msg {
        for (&_uid, tx) in users.read().await.iter() {
            tx.send(Message::Text(msg.clone()))
                .expect("Failed to send Message")
        }
    }
}

fn enrich_result(result: Message, id: usize) -> Result<Message, serde_json::Error> {
    match result {
        Message::Text(msg) => {
            let mut msg: Msg = serde_json::from_str(&msg)?;
            msg.uid = Some(id);
            let msg = serde_json::to_string(&msg)?;
            Ok(Message::Text(msg))
        }
        _ => Ok(result),
    }
}

async fn disconnect_user(
    Path(user_id): Path<usize>,
    Extension(users): Extension<Users>,
) -> impl IntoResponse {
    disconnect(user_id, &users).await;
    "Done"
}

// triggered when any user disconnects
async fn disconnect(my_id: usize, users: &Users) {
    println!("Good bye user {}", my_id);
    users.write().await.remove(&my_id);
    println!("Disconnected {my_id}");
}



