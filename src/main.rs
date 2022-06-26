use sdl_event_server::{SdlEvent, SdlReader, StdioInterface};
use crossbeam_channel::{tick, select, Receiver};

use std::{
    time::{self, Instant},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    }
};

const FREQUENCY: i32 = 60;

fn main() {
    let mut sdl_reader: SdlReader = SdlReader::new(); 

    // Our reader and stdio read from the same vector.
    let events_mtx: Arc<Mutex<Vec<SdlEvent>>> = Arc::new(Mutex::new(vec!()));
    let events_mtx_stdio = events_mtx.clone();

    // Both will run every 1/60s.
    let main_ticks: Receiver<Instant> = tick(time::Duration::from_secs_f32(1.0 / FREQUENCY as f32));
    let stdio_ticks: Receiver<Instant> = main_ticks.clone();

    // Both will also end at the same time.
    let main_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let stdio_done: Arc<AtomicBool> = main_done.clone();

    // We have to run stdio in parallel...
    let stdio_if: StdioInterface = StdioInterface::new(
        stdio_ticks, stdio_done, events_mtx_stdio
    );

    // ...but we MUST run SDL on the main thread.
    while !main_done.load(Ordering::Relaxed) {
        select! {
            recv(main_ticks) -> _ => {
                if let Ok(mut events) = events_mtx.lock() {
                    sdl_reader.poll_events(&mut events);
                }
            }
        }
    }

    stdio_if.join();
}
