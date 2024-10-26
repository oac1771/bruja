use clap::Parser;
use std::sync::{Arc, Mutex};
use tracing::Dispatch;

#[derive(Debug, Parser)]
pub struct Foo {}

struct BufferWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl std::io::Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Foo {
    pub async fn handle(&self) {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _join_handle = tokio::spawn(async move {
            // Setting up a default dispatcher for this task only.
            let subscriber = tracing_subscriber::fmt()
                .json()
                .with_writer(move || BufferWriter {
                    buffer: buffer.clone(),
                })
                .finish();

            let dispatch = Dispatch::new(subscriber);
            tracing::dispatcher::with_default(&dispatch, || {
                tracing::info!("task spawned");
            });
        });

        // Delay the main thread to allow the task to run.
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let foo = log_buffer.lock().unwrap().len();
        println!("log length: {:?}", foo);
    }
}
