use crate::sdl_event::SdlEvent;

use serde_json::json;
use std::{
    io::{BufWriter, Write},
    net::{TcpListener, TcpStream}
};

pub struct TcpWriter {
    listener: TcpListener,
    writer: Option<BufWriter<TcpStream>>
}

impl TcpWriter {
    pub fn new() -> TcpWriter {
        let listener: TcpListener = match TcpListener::bind("127.0.0.1:50404") {
            Err(e) => panic!("TCP listener failed to bind: {}", e),
            Ok(listener) => listener
        };
        return TcpWriter {
            listener: listener,
            writer: None
        }
    }

    pub fn write_events(&mut self, events: &mut Vec<SdlEvent>) -> Result<(), String> {
        match &mut self.writer {
            None => return Err(String::from("Attempted to write events without a client.")),
            Some(writer) => {
                while let Some(event) = events.pop() {
                    match writer.write(json!(event).to_string().as_bytes()) {
                        Err(e) => panic!("Failed to write to TCP writer: {}", e),
                        Ok(written) => {
                            if written == 0 {
                                return Err(String::from("TCP writer was closed."));
                            } else {
                                match writer.flush() {
                                    Err(_) => eprintln!("Failed to flush TCP writer."),
                                    Ok(_) => ()
                                }
                            }
                        }
                    }
                }
                return Ok(());
            }
        }
    }

    pub fn accept(&mut self) -> Result<(), String> {
        match self.listener.accept() {
            Ok((stream, _addr)) => {
                self.writer = Some(BufWriter::new(stream));
                return Ok(())
            },
            Err(e) => return Err(format!("Failed to accept client: {:?}", e))
        }
    }
}
