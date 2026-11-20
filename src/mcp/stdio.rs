use super::Router;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tokio::task::JoinSet;
const RESPONSE_CHANNEL_CAPACITY: usize = 64;
pub async fn run(router: Arc<Router>) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    let (response_tx, mut response_rx) = mpsc::channel::<Vec<u8>>(RESPONSE_CHANNEL_CAPACITY);
    let writer = tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(payload) = response_rx.recv().await {
            stdout.write_all(&payload).await?;
            stdout.flush().await?;
        }
        Ok::<(), std::io::Error>(())
    });
    let mut handlers = JoinSet::new();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let router = Arc::clone(&router);
        let response_tx = response_tx.clone();
        handlers.spawn(async move {
            if let Some(json) = router.handle_message(&line).await {
                let mut payload = serde_json::to_vec(&json).map_err(std::io::Error::other)?;
                payload.push(b'\n');
                response_tx.send(payload).await.map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::BrokenPipe, "stdout writer stopped")
                })?;
            }
            Ok::<(), std::io::Error>(())
        });
    }
    drop(response_tx);
    while let Some(result) = handlers.join_next().await {
        result??;
    }
    writer.await??;
    Ok(())
}
