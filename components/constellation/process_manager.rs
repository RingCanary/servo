/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::process::Child;

use crossbeam_channel::{Receiver, Select};
use log::{debug, warn};
#[cfg(unix)]
use nix::errno::Errno;
#[cfg(unix)]
use nix::sys::wait::waitpid;
#[cfg(unix)]
use nix::unistd::Pid;
use profile_traits::mem::{ProfilerChan, ProfilerMsg};

pub enum Process {
    Unsandboxed(Child),
    Sandboxed(u32),
}

impl Process {
    fn pid(&self) -> u32 {
        match self {
            Self::Unsandboxed(child) => child.id(),
            Self::Sandboxed(pid) => *pid,
        }
    }

    fn wait(&mut self) {
        match self {
            Self::Unsandboxed(child) => {
                let _ = child.wait();
            },
            Self::Sandboxed(pid) => {
                #[cfg(unix)]
                {
                    let pid = Pid::from_raw(*pid as i32);
                    loop {
                        match waitpid(pid, None) {
                            Ok(_) => {
                                debug!("Sandboxed process {} waited successfully", pid);
                                break;
                            },
                            Err(Errno::EINTR) => continue,
                            Err(e) => {
                                warn!("Failed to wait for sandboxed process {}: {}", pid, e);
                                break;
                            },
                        }
                    }
                }
                #[cfg(not(unix))]
                {
                    // TODO: use nix::waitpid() on supported platforms.
                    warn!(
                        "wait() is not yet implemented for sandboxed processes (pid={}).",
                        pid
                    );
                }
            },
        }
    }
}

type ProcessReceiver = Receiver<Result<(), ipc_channel::Error>>;

pub(crate) struct ProcessManager {
    processes: Vec<(Process, ProcessReceiver)>,
    mem_profiler_chan: ProfilerChan,
}

impl ProcessManager {
    pub fn new(mem_profiler_chan: ProfilerChan) -> Self {
        Self {
            processes: vec![],
            mem_profiler_chan,
        }
    }

    pub fn add(&mut self, receiver: ProcessReceiver, process: Process) {
        debug!("Adding process pid={}", process.pid());
        self.processes.push((process, receiver));
    }

    pub fn register<'a>(&'a self, select: &mut Select<'a>) {
        for (_, receiver) in &self.processes {
            select.recv(receiver);
        }
    }

    pub fn receiver_at(&self, index: usize) -> &ProcessReceiver {
        let (_, receiver) = &self.processes[index];
        receiver
    }

    pub fn remove(&mut self, index: usize) {
        let (mut process, _) = self.processes.swap_remove(index);
        debug!("Removing process pid={}", process.pid());
        // Unregister this process system memory profiler
        self.mem_profiler_chan
            .send(ProfilerMsg::UnregisterReporter(format!(
                "system-content-{}",
                process.pid()
            )));
        process.wait();
    }
}
