use std::env;
use std::ffi::OsString;
use std::io::Cursor;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use nix::unistd::{ForkResult, fork};

static DING_SAMPLE: &[u8] = include_bytes!("../ogg/ding.ogg");
static ERROR_SAMPLE: &[u8] = include_bytes!("../ogg/error.ogg");

const USAGE: &str = r#"Usage: ding [command]

Without a command, `ding` plays a happy ding sound.

With a command, `ding` will execute that command and play a ding if the
command completed without error, otherwise an error sound will be played.

If the command took more than five seconds to execute, the elapsed time will be
shown."#;

#[cfg(not(unix))]
compile_error!("This crate only supports Unix-like operating systems");

fn play_audio_and_wait(return_code: i32) {
    let data = if return_code == 0 {
        DING_SAMPLE
    } else {
        ERROR_SAMPLE
    };
    if let Ok(mut manager) = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        && let Ok(sound) = StaticSoundData::from_cursor(Cursor::new(data))
    {
        let sound_data = sound.with_settings(StaticSoundSettings::default());
        let duration = sound_data.duration();
        if let Err(e) = manager.play(sound_data) {
            eprintln!("unable to play sound: {e}");
        }
        sleep(duration);
    }
}

fn main() {
    let mut args = env::args_os().skip(1).peekable();

    // check for help flag
    if let Some(first_arg) = args.peek() {
        let first = first_arg.to_string_lossy();
        if first == "-h" || first == "--help" {
            println!("{USAGE}");
            return;
        }
    }

    let return_code = if let Some(command) = args.next() {
        let start = Instant::now();
        let remaining_args: Vec<OsString> = args.collect();

        let code = match std::process::Command::new(&command)
            .args(&remaining_args)
            .status()
        {
            Ok(status) => status.code().unwrap_or(1),
            Err(e) => {
                eprintln!("failed to execute {}: {e}", command.to_string_lossy());
                1
            }
        };

        let duration = start.elapsed();
        if duration.as_secs() > 5 {
            eprintln!(
                "Elapsed: {}",
                humantime::format_duration(Duration::from_secs(duration.as_secs()))
            );
        }

        code
    } else {
        0
    };

    // safety: fork is called before any multi-threaded code and both processes
    // immediately exit after the fork, avoiding any shared state issues
    unsafe {
        match fork() {
            Ok(ForkResult::Child) => {
                play_audio_and_wait(return_code);
                exit(0);
            }
            Ok(ForkResult::Parent { .. }) => exit(return_code),
            Err(err) => {
                eprintln!("fork failed: {err}");
                exit(1);
            }
        }
    }
}
