use std::io::Read;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Bound child execution while draining pipes so logging cannot block process exit.
pub fn run(command: &mut Command) -> Output {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn test child");
    let mut stdout = child.stdout.take().expect("child stdout pipe");
    let mut stderr = child.stderr.take().expect("child stderr pipe");
    let stdout_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).expect("read child stdout");
        bytes
    });
    let stderr_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).expect("read child stderr");
        bytes
    });

    let deadline = Instant::now() + Duration::from_secs(10);
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll test child") {
            break status;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            child.kill().expect("terminate timed-out child");
            break child.wait().expect("reap timed-out child");
        }
        thread::sleep(Duration::from_millis(10));
    };
    let stdout = stdout_reader.join().expect("join stdout reader");
    let stderr = stderr_reader.join().expect("join stderr reader");
    assert!(!timed_out, "test child exceeded its 10-second deadline");
    Output {
        status,
        stdout,
        stderr,
    }
}
