use serde::Deserialize;
use serde_json::Deserializer;
use serde_json::Value;
use std::{
    io::{Cursor, Write},
    sync::{Arc, Mutex},
};
use tokio::{
    select,
    time::{sleep, Duration},
};

pub struct BufferWriter {
    pub buffer: Arc<Mutex<Vec<u8>>>,
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[allow(async_fn_in_trait)]
pub trait Runner {
    fn label(&self) -> &str;

    fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>>;

    fn log_filter(&self, log: &Log) -> bool;

    async fn assert_log_entry(&self, entry: &str) {
        select! {
            _ = sleep(Duration::from_secs(5)) => {
                let log_buffer = self.log_buffer();
                let buffer = log_buffer.lock().unwrap();
                let output = String::from_utf8(buffer.clone()).unwrap();
                panic!("Failed to find log entry: {}\nLogs: {}", entry.to_string(), output)
            },
            _ = self.parse_logs(entry) => {}
        }
    }

    async fn parse_logs(&self, entry: &str) {
        let mut logs: Vec<Log> = vec![];

        while logs.len() == 0 {
            let log_buffer = self.log_buffer();
            let buffer = log_buffer.lock().unwrap();
            let log_output = String::from_utf8(buffer.clone()).unwrap();
            std::mem::drop(buffer);

            let cursor = Cursor::new(log_output);

            logs = Deserializer::from_reader(cursor.clone())
                .into_iter::<Log>()
                .map(|log| log.unwrap())
                .filter(|log| self.log_filter(log))
                .filter(|log| match &log.fields {
                    Fields::Message { message } => message == entry,
                    _ => false,
                })
                .collect::<Vec<Log>>();

            let _ = sleep(Duration::from_millis(100)).await;
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Log {
    fields: Fields,
    target: String,
    spans: Vec<Value>,
}

impl Log {
    pub fn target(&self) -> String {
        self.target.to_string()
    }

    pub fn spans(&self) -> &Vec<Value> {
        &self.spans
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Fields {
    #[allow(unused)]
    LocalPeerId {
        local_peer_id: String,
    },
    Message {
        message: String,
    },
}
