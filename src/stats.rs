use std::sync::{mpsc, Arc, Mutex};
use std::time;

#[derive(Debug)]
pub struct Stats {
    pub req_count: i32,
    pub errors_count: i32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            req_count: 0,
            errors_count: 0,
        }
    }
}

pub enum StatMessage {
    ResponseSuccess,
    ResponseFail,
}

pub struct StatsAggregator {
    stats: Arc<Mutex<Stats>>,
    interval: u64,
    channel: mpsc::Receiver<StatMessage>,
}

impl StatsAggregator {
    pub fn new(interval: u64, channel: mpsc::Receiver<StatMessage>) -> StatsAggregator {
        let stats = Arc::new(Mutex::new(Stats::new()));

        StatsAggregator {
            stats, interval, channel,
        }
    }

    pub fn run(&mut self) {
        let shared_stats = Arc::clone(&self.stats);
        let tick_interval = self.interval;

        std::thread::spawn(move || {
            loop {
                std::thread::sleep(time::Duration::from_secs(tick_interval));
                let mut app_stats = shared_stats.lock().unwrap();
                println!("Got stats {:#?} for {} seconds", app_stats, tick_interval);
                app_stats.req_count = 0;
                app_stats.errors_count = 0;
            }
        });

        loop {
            match self.channel.recv() {
                Ok(message) => self.process_message(message),
                Err(_) => break,
            };
        };
    }

    pub fn process_message(&mut self, message: StatMessage) {
        let mut captured_stats = self.stats.lock().unwrap();
        match message {
            StatMessage::ResponseSuccess => {
                captured_stats.req_count += 1;
            },
            StatMessage::ResponseFail => {
                captured_stats.req_count += 1;
                captured_stats.errors_count += 1;
            }
        };
    }
}