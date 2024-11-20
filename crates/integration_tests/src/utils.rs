use serde::{Deserialize, Serialize};
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
    fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>>;

    fn log_filter(&self, log: &Log) -> bool;

    fn get_logs(&self) -> Vec<Log> {
        let log_buffer = self.log_buffer();
        let buffer = log_buffer.lock().unwrap();
        let log_output = String::from_utf8(buffer.clone()).unwrap();
        let cursor = Cursor::new(log_output);
        let logs = Deserializer::from_reader(cursor.clone())
            .into_iter::<Log>()
            .map(|log| log.unwrap())
            .collect::<Vec<Log>>();

        logs
    }

    fn log_output(&self) -> String {
        let logs = self.get_logs();
        let output = serde_json::to_string_pretty(&logs).unwrap();

        output
    }

    async fn assert_info_log_entry(&self, entry: &str) {
        self.assert_log(entry, tracing::Level::INFO, Eval::Equals)
            .await;
    }

    async fn assert_info_log_contains(&self, entry: &str) {
        self.assert_log(entry, tracing::Level::INFO, Eval::Contains)
            .await;
    }

    async fn assert_error_log_entry(&self, entry: &str) {
        self.assert_log(entry, tracing::Level::ERROR, Eval::Equals)
            .await;
    }

    async fn assert_log(&self, entry: &str, level: tracing::Level, eval: Eval) {
        let predicate: Box<dyn Fn(&String) -> bool> = match eval {
            Eval::Contains => {
                let entry_clone = entry.to_string();
                Box::new(move |msg: &String| msg.contains(&entry_clone))
            }
            Eval::Equals => {
                let entry_clone = entry.to_string();
                Box::new(move |msg: &String| msg == &entry_clone)
            }
        };

        select! {
            _ = sleep(Duration::from_secs(10)) => {
                let output = self.log_output();
                panic!("Logs: {}\nFailed to find log entry: {}", output, entry)
            },
            _ = self.parse_logs(predicate, level) => {}
        }
    }

    async fn parse_logs(&self, predicate: Box<dyn Fn(&String) -> bool>, level: tracing::Level) {
        let mut logs: Vec<Log> = vec![];

        while logs.len() == 0 {
            logs = self
                .get_logs()
                .into_iter()
                .filter(|log| log.level == level.as_str())
                .filter(|log| self.log_filter(log))
                .filter(|log| match &log.fields {
                    Fields::Message { message } => predicate(message),
                    _ => false,
                })
                .collect::<Vec<Log>>();

            let _ = sleep(Duration::from_millis(100)).await;
        }
    }
}

pub enum Eval {
    Contains,
    Equals,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Log {
    fields: Fields,
    target: String,
    spans: Vec<Value>,
    pub level: String,
}

impl Log {
    pub fn target(&self) -> String {
        self.target.to_string()
    }

    pub fn spans(&self) -> &Vec<Value> {
        &self.spans
    }
}

#[derive(Deserialize, Serialize, Debug)]
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
