pub mod sdl_event;
pub mod sdl_reader;
pub mod tcp_writer;

use crate::{
    sdl_event::SdlEvent,
    sdl_reader::SdlReader,
    tcp_writer::TcpWriter
};

use crossbeam_channel::{tick, select};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering}
};
use std::{thread, time};

fn main() {
    let mut sdl_reader: SdlReader = SdlReader::new();
    let mut tcp_writer: TcpWriter = TcpWriter::new();

    // Both our main and other thread will read from the same vector.
    let events_mtx: Arc<Mutex<Vec<SdlEvent>>> = Arc::new(Mutex::new(vec!()));
    let other_mtx = events_mtx.clone();

    // Both threads will run every 1/60s.
    let ticks = tick(time::Duration::from_secs_f32(1.0 / 60.0));
    let other_ticks = ticks.clone();

    // Both threads will also end at the same time.
    let threads_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let other_done: Arc<AtomicBool> = threads_done.clone();

    // Run our TCP writer on another thread.
    let tcp_thread: thread::JoinHandle<_> = thread::spawn(move || {
        match tcp_writer.accept() {
            Err(e) => eprintln!("{}", e),
            Ok(_) => ()
        }
        while !threads_done.load(Ordering::Relaxed) {
            select! {
                recv(ticks) -> _ => {
                    if let Ok(mut events) = events_mtx.lock() {
                        match tcp_writer.write_events(&mut events) {
                            Err(e) => {
                                eprintln!("{}", e);
                                threads_done.store(true, Ordering::SeqCst);
                            },
                            Ok(_) => ()
                        }
                    }
                }
            }
       }
    });

    // We MUST run SDL on the main thread.
    while !other_done.load(Ordering::Relaxed) {
        select! {
            recv(other_ticks) -> _ => {
                if let Ok(mut events) = other_mtx.lock() {
                    sdl_reader.poll_events(&mut events);
                }
            }
        }
    }

    match tcp_thread.join() {
        Err(_) => panic!("Failed to join the TCP thread."),
        Ok(_) => ()
    }
}
