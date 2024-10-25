use clap::Parser;
use std::sync::{Arc, Mutex};
use tracing_subscriber::util::SubscriberInitExt;

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
            let _guard = tracing_subscriber::fmt()
                .json()
                .with_writer(move || BufferWriter {
                    buffer: buffer.clone(),
                })
                .set_default();

            tracing::info!("task spawned");
        });
        tracing::info!("outside");
        let foo = log_buffer.lock().unwrap().len();
        println!("log length: {:?}", foo);
    }
}
