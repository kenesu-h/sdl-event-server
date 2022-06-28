use std::{
    io,
    io::{BufReader, BufRead, BufWriter, Write},
    thread,
    time::Instant,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    }
};

use crate::sdl_event::SdlEvent;
use crossbeam_channel::{select, Receiver};
use serde_json::json;

struct StdinReader {
    reader: BufReader<io::Stdin>,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        return StdinReader { reader: BufReader::new(io::stdin()) }
    }

    pub fn read(&mut self, done: Arc<AtomicBool>) -> () {
        let mut buffer: String = String::new();
        match self.reader.read_line(&mut buffer) {
            Err(_) => done.store(true, Ordering::SeqCst),
            Ok(_) => ()
        }
        match buffer.trim() {
            "exit" => done.store(true, Ordering::SeqCst),
            _ => ()
        }
    }
}

struct StdoutWriter {
    writer: BufWriter<io::Stdout>
}

impl StdoutWriter {
    pub fn new() -> StdoutWriter {
        return StdoutWriter {
            writer: BufWriter::new(io::stdout())
        }
    }

    pub fn write(
        &mut self, events_mtx: Arc<Mutex<Vec<SdlEvent>>>
    ) -> Result<(), String> {
        if let Ok(mut events) = events_mtx.lock() {
            while let Some(event) = events.pop() {
                let mut json_string: String = json!(event).to_string();
                json_string.push('\n');
                let json: &[u8] = json_string.as_bytes();
                if let Err(_) = self.writer.write(json) {
                    return Err(String::from("Failed to write to buffer."));
                } else {
                    if let Err(_) = self.writer.flush() {
                        return Err(String::from("Failed to flush buffer."));
                    }
                }
            }
        }
        return Ok(());
    }
}

pub struct StdioInterface {
    read_thread: thread::JoinHandle<()>,
    write_thread: thread::JoinHandle<()>
}

impl StdioInterface {
    pub fn new(
        ticks: Receiver<Instant>, done: Arc<AtomicBool>,
        events_mtx: Arc<Mutex<Vec<SdlEvent>>>
    ) -> StdioInterface {
        let mut reader: StdinReader = StdinReader::new();
        let mut writer: StdoutWriter = StdoutWriter::new();

        let read_ticks: Receiver<Instant> = ticks.clone();
        let write_ticks: Receiver<Instant> = ticks.clone();

        let read_done: Arc<AtomicBool> = Arc::clone(&done);
        let write_done: Arc<AtomicBool> = Arc::clone(&done);

        let read_thread: thread::JoinHandle<()> = thread::spawn(move || {
            while !read_done.load(Ordering::Relaxed) {
                select! {
                    recv(read_ticks) -> _ => reader.read(read_done.clone())
                }
            }
        });

        let write_thread: thread::JoinHandle<()> = thread::spawn(move || {
            while !write_done.load(Ordering::Relaxed) {
                select! {
                    recv(write_ticks) -> _ => match writer.write(
                        events_mtx.clone()
                    ) {
                        Err(_) => write_done.store(true, Ordering::SeqCst),
                        Ok(_) => ()
                    }                
                }
            }
        });
        return StdioInterface {
            read_thread: read_thread,
            write_thread: write_thread
        }
    }

    pub fn join(self) -> () {
        self.read_thread.join().expect("Failed to join read thread.");
        self.write_thread.join().expect("Failed to join write thread.");
    }
}
