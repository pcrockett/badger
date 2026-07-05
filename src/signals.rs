use std::sync::{Arc, atomic};

use anyhow::Result;
use nix::sys::signal as nix_signal;
use nix::unistd::Pid;
use signal_hook::consts::*;
use signal_hook::iterator::Signals;

/// Start a background thread that waits for signals and forwards them to a child
/// process. There is no mechanism to stop the thread, because so far we haven't needed
/// it. It's fine to stop when the program ends.
///
/// The thread will keep an eye on `child_pid`, and only forward signals as long as it
/// is non-zero.
pub fn forward_to(child_pid: Arc<atomic::AtomicU32>) -> Result<()> {
    const SIGNALS: [i32; 22] = [
        SIGABRT, SIGALRM, SIGBUS, SIGCONT, SIGHUP, SIGINT, SIGPIPE, SIGPROF, SIGQUIT, SIGSYS,
        SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH,
        SIGXCPU, SIGXFSZ,
    ];
    let mut signals = Signals::new(SIGNALS)?;
    std::thread::spawn(move || {
        for signal in signals.forever() {
            let pid = &child_pid.load(atomic::Ordering::Relaxed);
            if *pid == 0 {
                continue;
            }
            let pid = Pid::from_raw(*pid as i32);
            let signal = nix_signal::Signal::try_from(signal).expect("invalid signal");
            nix_signal::kill(pid, signal).expect("failed to send signal");
        }
    });

    Ok(())
}
