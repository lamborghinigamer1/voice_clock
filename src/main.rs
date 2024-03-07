use chrono::prelude::*;
use core::time;
use rodio::Decoder;
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

struct HourMinute {
    pub hour: u32,
    pub minute: u32,
}

fn get_current_time() -> HourMinute {
    let time: DateTime<Local> = Local::now();
    HourMinute {
        hour: time.hour(),
        minute: time.minute(),
    }
}

fn main() {
    let mut test = true;
    loop {
        let mut audio_files: Vec<String> = Vec::new();

        let time: HourMinute = get_current_time();

        if time.minute == 30 || time.minute == 0 || test {
            let mut hour = time.hour % 12;
            if hour == 0 {
                hour = 12;
            }

            let hour_file = format!("sounds/{}.mp3", hour);
            audio_files.push(hour_file);

            if time.minute == 30 {
                let minute_file = format!("sounds/30.mp3");
                audio_files.push(minute_file);
            }

            let period_file = if time.hour >= 12 {
                "sounds/pm.mp3".to_string()
            } else {
                "sounds/am.mp3".to_string()
            };
            audio_files.push(period_file);

            for audio_file in audio_files {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();

                // Open the MP3 file
                println!("{}", audio_file);
                let file = File::open(audio_file).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();

                // Append the MP3 decoder to the sink
                sink.append(source);
                sink.set_volume(1.0);
                // The sound plays in a separate thread. This call will block the current thread until the sink
                // has finished playing all its queued sounds.
                sink.sleep_until_end();
            }

            // Calculate remaining seconds in the current half-hour period
            let current_time = Local::now();
            let remaining_seconds = if current_time.minute() == 0 {
                30 * 60 - current_time.second()
            } else {
                60 * 60 - current_time.minute() * 60 - current_time.second()
            };

            println!("sleeping for: {}", remaining_seconds);

            // Sleep for the remaining time of the current half-hour period
            thread::sleep(time::Duration::from_secs(remaining_seconds as u64));
        }
        test = false;
    }
}
