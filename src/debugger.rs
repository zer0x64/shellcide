#![cfg_attr(target_arch = "wasm32", allow(dead_code, unused_imports))]

use flume::{Receiver, Sender};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use nix::sys::ptrace;
#[cfg(not(target_arch = "wasm32"))]
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
#[cfg(not(target_arch = "wasm32"))]
use std::fs::{File, OpenOptions};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Seek, SeekFrom, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::os::unix::io::FromRawFd;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(not(target_arch = "wasm32"))]
pub use nix::unistd::Pid;
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid(pub i32);

#[cfg(not(target_arch = "wasm32"))]
pub use nix::sys::signal::Signal;
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    SIGTRAP,
    SIGSEGV,
    SIGILL,
    SIGFPE,
    SIGKILL,
    SIGSTOP,
}

pub const CODE_BASE: usize = 0x1000_0000;
pub const DATA_BASE: usize = 0x2000_0000;
pub const STACK_BASE: usize = 0x3000_0000;
pub const STACK_SIZE: usize = 0x10_0000; // 1MB

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CpuRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl CpuRegisters {
    pub fn get_by_name(&self, name: &str) -> Option<u64> {
        match name {
            "rax" => Some(self.rax),
            "rbx" => Some(self.rbx),
            "rcx" => Some(self.rcx),
            "rdx" => Some(self.rdx),
            "rsi" => Some(self.rsi),
            "rdi" => Some(self.rdi),
            "rbp" => Some(self.rbp),
            "rsp" => Some(self.rsp),
            "rip" => Some(self.rip),
            "rflags" => Some(self.rflags),
            "r8" => Some(self.r8),
            "r9" => Some(self.r9),
            "r10" => Some(self.r10),
            "r11" => Some(self.r11),
            "r12" => Some(self.r12),
            "r13" => Some(self.r13),
            "r14" => Some(self.r14),
            "r15" => Some(self.r15),
            _ => None,
        }
    }

    pub fn set_by_name(&mut self, name: &str, val: u64) -> bool {
        match name {
            "rax" => self.rax = val,
            "rbx" => self.rbx = val,
            "rcx" => self.rcx = val,
            "rdx" => self.rdx = val,
            "rsi" => self.rsi = val,
            "rdi" => self.rdi = val,
            "rbp" => self.rbp = val,
            "rsp" => self.rsp = val,
            "rip" => self.rip = val,
            "rflags" => self.rflags = val,
            "r8" => self.r8 = val,
            "r9" => self.r9 = val,
            "r10" => self.r10 = val,
            "r11" => self.r11 = val,
            "r12" => self.r12 = val,
            "r13" => self.r13 = val,
            "r14" => self.r14 = val,
            "r15" => self.r15 = val,
            _ => return false,
        }
        true
    }
}

pub enum DebuggerCommand {
    Start {
        code: Vec<u8>,
        initial_regs: CpuRegisters,
        memory_patches: Vec<(usize, u8)>,
        breakpoints: Vec<usize>,
    },
    Step,
    Continue,
    WriteRegs(CpuRegisters),
    WriteMemory(usize, Vec<u8>),
    ToggleBreakpoint(usize, bool), // (address, enable)
    Pause,
    Terminate,
}

#[derive(Debug, Clone)]
pub enum DebuggerEvent {
    Started {
        pid: Pid,
        regs: CpuRegisters,
    },
    Stopped {
        regs: CpuRegisters,
        signal: Signal,
        trap_addr: Option<usize>, // Address of breakpoint if hit
        status_message: String,
    },
    Terminated {
        exit_code: i32,
        status_message: String,
    },
    Signaled {
        signal: Signal,
        status_message: String,
    },
    Stdout(String),
    Stderr(String),
    Error(String),
    #[allow(dead_code)]
    FileLoaded {
        filename: String,
        content: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitResult {
    Exited,
    StoppedSignal,
    BreakpointHit(usize),
    Terminated,
}

/// Sets up the page mappings in the parent process using `mmap`.
/// These are inherited as Copy-on-Write by the child.
#[cfg(not(target_arch = "wasm32"))]
pub fn setup_parent_mappings() -> Result<(), String> {
    unsafe {
        // Map Code Segment
        let code_ptr = libc::mmap(
            CODE_BASE as *mut libc::c_void,
            0x10_0000, // 1MB
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
            -1,
            0,
        );
        if code_ptr == libc::MAP_FAILED {
            return Err("Failed to map Code segment".to_string());
        }

        // Map Data Segment
        let data_ptr = libc::mmap(
            DATA_BASE as *mut libc::c_void,
            0x10_0000, // 1MB
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
            -1,
            0,
        );
        if data_ptr == libc::MAP_FAILED {
            return Err("Failed to map Data segment".to_string());
        }

        // Map Stack Segment
        let stack_ptr = libc::mmap(
            STACK_BASE as *mut libc::c_void,
            STACK_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
            -1,
            0,
        );
        if stack_ptr == libc::MAP_FAILED {
            return Err("Failed to map Stack segment".to_string());
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn setup_parent_mappings() -> Result<(), String> {
    Ok(())
}

/// Reads memory of the child process.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_child_mem(pid: Pid, address: usize, len: usize) -> std::io::Result<Vec<u8>> {
    let mut segment_end = None;
    if (CODE_BASE..CODE_BASE + 0x10_0000).contains(&address) {
        segment_end = Some(CODE_BASE + 0x10_0000);
    } else if (DATA_BASE..DATA_BASE + 0x10_0000).contains(&address) {
        segment_end = Some(DATA_BASE + 0x10_0000);
    } else if (STACK_BASE..STACK_BASE + STACK_SIZE).contains(&address) {
        segment_end = Some(STACK_BASE + STACK_SIZE);
    }

    let path = format!("/proc/{}/mem", pid);
    let mut file = File::open(path)?;

    if let Some(end) = segment_end {
        if len > end - address {
            let clamped_len = end - address;
            file.seek(SeekFrom::Start(address as u64))?;
            let mut buf = vec![0; clamped_len];
            file.read_exact(&mut buf)?;
            buf.resize(len, 0);
            return Ok(buf);
        }
    }

    file.seek(SeekFrom::Start(address as u64))?;
    let mut buf = vec![0; len];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

#[cfg(target_arch = "wasm32")]
pub fn read_child_mem(_pid: Pid, _address: usize, len: usize) -> std::io::Result<Vec<u8>> {
    Ok(vec![0; len])
}

/// Writes memory of the child process.
#[cfg(not(target_arch = "wasm32"))]
pub fn write_child_mem(pid: Pid, address: usize, bytes: &[u8]) -> std::io::Result<()> {
    let path = format!("/proc/{}/mem", pid);
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.seek(SeekFrom::Start(address as u64))?;
    file.write_all(bytes)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn write_child_mem(_pid: Pid, _address: usize, _bytes: &[u8]) -> std::io::Result<()> {
    Ok(())
}

/// Reads the child's registers.
#[cfg(not(target_arch = "wasm32"))]
pub fn get_child_regs(pid: Pid) -> Result<CpuRegisters, String> {
    let regs = ptrace::getregs(pid).map_err(|e| format!("Failed to get registers: {}", e))?;
    Ok(CpuRegisters {
        rax: regs.rax,
        rbx: regs.rbx,
        rcx: regs.rcx,
        rdx: regs.rdx,
        rsi: regs.rsi,
        rdi: regs.rdi,
        rbp: regs.rbp,
        rsp: regs.rsp,
        rip: regs.rip,
        rflags: regs.eflags,
        r8: regs.r8,
        r9: regs.r9,
        r10: regs.r10,
        r11: regs.r11,
        r12: regs.r12,
        r13: regs.r13,
        r14: regs.r14,
        r15: regs.r15,
    })
}

#[cfg(target_arch = "wasm32")]
pub fn get_child_regs(_pid: Pid) -> Result<CpuRegisters, String> {
    Ok(CpuRegisters::default())
}

/// Writes the child's registers.
#[cfg(not(target_arch = "wasm32"))]
pub fn set_child_regs(pid: Pid, cpu_regs: CpuRegisters) -> Result<(), String> {
    let mut regs = ptrace::getregs(pid).map_err(|e| format!("Failed to get registers: {}", e))?;
    regs.rax = cpu_regs.rax;
    regs.rbx = cpu_regs.rbx;
    regs.rcx = cpu_regs.rcx;
    regs.rdx = cpu_regs.rdx;
    regs.rsi = cpu_regs.rsi;
    regs.rdi = cpu_regs.rdi;
    regs.rbp = cpu_regs.rbp;
    regs.rsp = cpu_regs.rsp;
    regs.rip = cpu_regs.rip;
    regs.eflags = cpu_regs.rflags;
    regs.r8 = cpu_regs.r8;
    regs.r9 = cpu_regs.r9;
    regs.r10 = cpu_regs.r10;
    regs.r11 = cpu_regs.r11;
    regs.r12 = cpu_regs.r12;
    regs.r13 = cpu_regs.r13;
    regs.r14 = cpu_regs.r14;
    regs.r15 = cpu_regs.r15;
    ptrace::setregs(pid, regs).map_err(|e| format!("Failed to set registers: {}", e))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn set_child_regs(_pid: Pid, _cpu_regs: CpuRegisters) -> Result<(), String> {
    Ok(())
}

/// Spawns a background thread to read from a redirected pipe descriptor.
#[cfg(not(target_arch = "wasm32"))]
fn spawn_pipe_reader(fd: std::os::unix::io::RawFd, tx: Sender<DebuggerEvent>, is_stderr: bool) {
    thread::spawn(move || {
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut buf = [0; 1024];
        loop {
            match file.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let s = String::from_utf8_lossy(&buf[..n]).to_string();
                    if is_stderr {
                        let _ = tx.send(DebuggerEvent::Stderr(s));
                    } else {
                        let _ = tx.send(DebuggerEvent::Stdout(s));
                    }
                }
                Err(_) => break,
            }
        }
    });
}

/// Main debugger thread execution loop.
#[cfg(not(target_arch = "wasm32"))]
pub fn run_debugger_thread(rx: Receiver<DebuggerCommand>, tx: Sender<DebuggerEvent>) {
    let mut child_pid: Option<Pid> = None;
    let mut breakpoints: HashMap<usize, u8> = HashMap::new(); // Address -> Original Byte
    let mut last_breakpoint_addr: Option<usize> = None;
    let mut user_code_len = 0;

    // Helper closure to write breakpoint INT3 bytes
    let apply_breakpoints = |pid: Pid, bps: &[usize], bp_map: &mut HashMap<usize, u8>| {
        for &addr in bps {
            if let Ok(orig) = read_child_mem(pid, addr, 1) {
                bp_map.insert(addr, orig[0]);
                let _ = write_child_mem(pid, addr, &[0xCC]); // Write INT3
            }
        }
    };

    while let Ok(cmd) = rx.recv() {
        match cmd {
            DebuggerCommand::Start {
                code,
                initial_regs,
                memory_patches,
                breakpoints: initial_bps,
            } => {
                // Clean up previous child
                if let Some(pid) = child_pid {
                    let _ = nix::sys::signal::kill(pid, Signal::SIGKILL);
                    let _ = waitpid(pid, None);
                }

                breakpoints.clear();
                last_breakpoint_addr = None;
                user_code_len = code.len();

                // Build safety exit loop at end of code block: INT3 (0xCC)
                let mut full_code = code.clone();
                full_code.push(0xCC); // Safety trap byte

                // Setup output pipes
                let (stdout_r, stdout_w) = nix::unistd::pipe().unwrap();
                let (stderr_r, stderr_w) = nix::unistd::pipe().unwrap();

                match unsafe { nix::unistd::fork() } {
                    Ok(nix::unistd::ForkResult::Child) => {
                        // Redirect standard streams
                        nix::unistd::dup2(stdout_w, 1).unwrap();
                        nix::unistd::dup2(stderr_w, 2).unwrap();

                        // Close unused descriptors
                        nix::unistd::close(stdout_r).unwrap();
                        nix::unistd::close(stdout_w).unwrap();
                        nix::unistd::close(stderr_r).unwrap();
                        nix::unistd::close(stderr_w).unwrap();

                        // Register for tracing and stop
                        ptrace::traceme().expect("ptrace traceme failed");
                        nix::sys::signal::raise(Signal::SIGSTOP).expect("raise failed");

                        // Fallback execution loop if tracee runs out of bounds
                        loop {
                            thread::sleep(std::time::Duration::from_secs(1));
                        }
                    }
                    Ok(nix::unistd::ForkResult::Parent { child }) => {
                        child_pid = Some(child);

                        // Close unused write ends in parent
                        nix::unistd::close(stdout_w).unwrap();
                        nix::unistd::close(stderr_w).unwrap();

                        // Spawn output readers
                        spawn_pipe_reader(stdout_r, tx.clone(), false);
                        spawn_pipe_reader(stderr_r, tx.clone(), true);

                        // Wait for initial stop
                        match waitpid(child, None) {
                            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                                // Write compiled code into child space
                                if let Err(e) = write_child_mem(child, CODE_BASE, &full_code) {
                                    let _ = tx.send(DebuggerEvent::Error(format!(
                                        "Memory write failed: {}",
                                        e
                                    )));
                                    let _ = nix::sys::signal::kill(child, Signal::SIGKILL);
                                    let _ = waitpid(child, None);
                                    child_pid = None;
                                    continue;
                                }

                                // Apply memory patches
                                for (addr, val) in memory_patches {
                                    if let Err(e) = write_child_mem(child, addr, &[val]) {
                                        let _ = tx.send(DebuggerEvent::Error(format!(
                                            "Initial memory patch failed: {}",
                                            e
                                        )));
                                    }
                                }

                                // Apply initial breakpoints
                                apply_breakpoints(child, &initial_bps, &mut breakpoints);

                                // Configure registers
                                let mut regs = initial_regs;
                                regs.rip = CODE_BASE as u64;
                                regs.rsp = (STACK_BASE + STACK_SIZE - 8) as u64; // Set initial stack pointer
                                if let Err(e) = set_child_regs(child, regs) {
                                    let _ = tx.send(DebuggerEvent::Error(format!(
                                        "Register write failed: {}",
                                        e
                                    )));
                                    let _ = nix::sys::signal::kill(child, Signal::SIGKILL);
                                    let _ = waitpid(child, None);
                                    child_pid = None;
                                    continue;
                                }

                                let _ = tx.send(DebuggerEvent::Started { pid: child, regs });

                                // Resume child execution immediately so there is no automatic breakpoint on the first instruction!
                                if let Err(e) = ptrace::cont(child, None) {
                                    let _ = tx.send(DebuggerEvent::Error(format!(
                                        "Failed to continue execution: {}",
                                        e
                                    )));
                                    let _ = nix::sys::signal::kill(child, Signal::SIGKILL);
                                    let _ = waitpid(child, None);
                                    child_pid = None;
                                    continue;
                                }
                                wait_and_handle(
                                    child,
                                    &tx,
                                    &mut breakpoints,
                                    &mut last_breakpoint_addr,
                                    user_code_len,
                                    &rx,
                                );
                            }
                            other => {
                                let _ = tx.send(DebuggerEvent::Error(format!(
                                    "Unexpected child startup state: {:?}",
                                    other
                                )));
                                let _ = nix::sys::signal::kill(child, Signal::SIGKILL);
                                let _ = waitpid(child, None);
                                child_pid = None;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(DebuggerEvent::Error(format!("Fork failed: {}", e)));
                    }
                }
            }
            DebuggerCommand::Step => {
                if let Some(pid) = child_pid {
                    // Step over breakpoint if we are currently parked on one
                    if let Some(bp_addr) = last_breakpoint_addr {
                        // Restore original byte, step once, put INT3 back
                        if let Some(&orig_byte) = breakpoints.get(&bp_addr) {
                            let _ = write_child_mem(pid, bp_addr, &[orig_byte]);
                        }
                        if let Err(e) = ptrace::step(pid, None) {
                            let _ =
                                tx.send(DebuggerEvent::Error(format!("Step-over failed: {}", e)));
                            continue;
                        }
                        let wait_res = wait_and_handle(
                            pid,
                            &tx,
                            &mut breakpoints,
                            &mut last_breakpoint_addr,
                            user_code_len,
                            &rx,
                        );
                        // Re-write breakpoint INT3 byte
                        let _ = write_child_mem(pid, bp_addr, &[0xCC]);

                        if wait_res == WaitResult::StoppedSignal {
                            last_breakpoint_addr = None;
                        }
                        continue;
                    }

                    // Perform the actual step command
                    if let Err(e) = ptrace::step(pid, None) {
                        let _ = tx.send(DebuggerEvent::Error(format!("Step failed: {}", e)));
                        continue;
                    }
                    let wait_res = wait_and_handle(
                        pid,
                        &tx,
                        &mut breakpoints,
                        &mut last_breakpoint_addr,
                        user_code_len,
                        &rx,
                    );
                    if wait_res == WaitResult::StoppedSignal {
                        last_breakpoint_addr = None;
                    }
                }
            }
            DebuggerCommand::Continue => {
                if let Some(pid) = child_pid {
                    let mut should_continue = true;
                    if let Some(bp_addr) = last_breakpoint_addr {
                        if let Some(&orig_byte) = breakpoints.get(&bp_addr) {
                            let _ = write_child_mem(pid, bp_addr, &[orig_byte]);
                        }
                        if let Err(e) = ptrace::step(pid, None) {
                            let _ = tx.send(DebuggerEvent::Error(format!(
                                "Step-over continue failed: {}",
                                e
                            )));
                            continue;
                        }
                        let wait_res = wait_and_handle(
                            pid,
                            &tx,
                            &mut breakpoints,
                            &mut last_breakpoint_addr,
                            user_code_len,
                            &rx,
                        );
                        // Put INT3 back
                        let _ = write_child_mem(pid, bp_addr, &[0xCC]);

                        match wait_res {
                            WaitResult::StoppedSignal => {
                                last_breakpoint_addr = None;
                            }
                            WaitResult::BreakpointHit(_) => {
                                should_continue = false;
                            }
                            _ => {
                                should_continue = false;
                            }
                        }
                    }

                    if should_continue {
                        if let Err(e) = ptrace::cont(pid, None) {
                            let _ =
                                tx.send(DebuggerEvent::Error(format!("Continue failed: {}", e)));
                            continue;
                        }
                        let _wait_res = wait_and_handle(
                            pid,
                            &tx,
                            &mut breakpoints,
                            &mut last_breakpoint_addr,
                            user_code_len,
                            &rx,
                        );
                    }
                }
            }
            DebuggerCommand::WriteRegs(regs) => {
                if let Some(pid) = child_pid {
                    if let Err(e) = set_child_regs(pid, regs) {
                        let _ = tx.send(DebuggerEvent::Error(format!(
                            "Failed to write registers: {}",
                            e
                        )));
                    }
                }
            }
            DebuggerCommand::WriteMemory(addr, data) => {
                if let Some(pid) = child_pid {
                    if let Err(e) = write_child_mem(pid, addr, &data) {
                        let _ = tx.send(DebuggerEvent::Error(format!(
                            "Failed to write memory: {}",
                            e
                        )));
                    }
                }
            }
            DebuggerCommand::ToggleBreakpoint(addr, enable) => {
                if enable {
                    if let Some(pid) = child_pid {
                        if let Ok(orig) = read_child_mem(pid, addr, 1) {
                            breakpoints.insert(addr, orig[0]);
                            let _ = write_child_mem(pid, addr, &[0xCC]);
                        }
                    }
                } else {
                    if let Some(pid) = child_pid {
                        if let Some(orig_byte) = breakpoints.remove(&addr) {
                            let _ = write_child_mem(pid, addr, &[orig_byte]);
                        }
                    }
                }
            }
            DebuggerCommand::Pause => {
                if let Some(pid) = child_pid {
                    let _ = nix::sys::signal::kill(pid, Signal::SIGSTOP);
                }
            }
            DebuggerCommand::Terminate => {
                if let Some(pid) = child_pid {
                    let _ = nix::sys::signal::kill(pid, Signal::SIGKILL);
                    let _ = waitpid(pid, None);
                    child_pid = None;
                    let _ = tx.send(DebuggerEvent::Terminated {
                        exit_code: -1,
                        status_message: "Process terminated by user.".to_string(),
                    });
                }
            }
        }
    }
}

/// Waits for a child state change and reports it to the GUI.
#[cfg(not(target_arch = "wasm32"))]
fn wait_and_handle(
    pid: Pid,
    tx: &Sender<DebuggerEvent>,
    breakpoints: &mut HashMap<usize, u8>,
    last_breakpoint_addr: &mut Option<usize>,
    user_code_len: usize,
    rx: &Receiver<DebuggerCommand>,
) -> WaitResult {
    loop {
        // Check for incoming control commands while waiting
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                DebuggerCommand::Pause => {
                    let _ = nix::sys::signal::kill(pid, Signal::SIGSTOP);
                }
                DebuggerCommand::Terminate => {
                    let _ = nix::sys::signal::kill(pid, Signal::SIGKILL);
                    let _ = waitpid(pid, None);
                    let _ = tx.send(DebuggerEvent::Terminated {
                        exit_code: -1,
                        status_message: "Process terminated by user.".to_string(),
                    });
                    return WaitResult::Terminated;
                }
                DebuggerCommand::ToggleBreakpoint(addr, enable) => {
                    if enable {
                        if let Ok(orig) = read_child_mem(pid, addr, 1) {
                            breakpoints.insert(addr, orig[0]);
                            let _ = write_child_mem(pid, addr, &[0xCC]);
                        }
                    } else {
                        if let Some(orig_byte) = breakpoints.remove(&addr) {
                            let _ = write_child_mem(pid, addr, &[orig_byte]);
                        }
                    }
                }
                _ => {} // Ignore other commands while running
            }
        }

        match waitpid(pid, Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::StillAlive) => {
                thread::sleep(std::time::Duration::from_millis(5));
            }
            Ok(WaitStatus::Stopped(_, sig)) => {
                let mut regs = match get_child_regs(pid) {
                    Ok(r) => r,
                    Err(e) => {
                        let _ = tx.send(DebuggerEvent::Error(format!(
                            "Failed to read registers: {}",
                            e
                        )));
                        return WaitResult::Exited;
                    }
                };
                let mut trap_addr = None;
                let mut status_message = format!("Stopped by signal {:?}", sig);
                let mut is_bp = false;

                if sig == Signal::SIGTRAP {
                    let possible_bp = (regs.rip - 1) as usize;

                    // Case 1: Hit safety exit loop trap
                    if possible_bp == CODE_BASE + user_code_len {
                        regs.rip = possible_bp as u64;
                        let _ = set_child_regs(pid, regs);
                        let _ = tx.send(DebuggerEvent::Terminated {
                            exit_code: 0,
                            status_message: "Program execution completed successfully.".to_string(),
                        });
                        return WaitResult::Exited;
                    }

                    // Case 2: Hit a user-defined breakpoint
                    if breakpoints.contains_key(&possible_bp) {
                        regs.rip = possible_bp as u64;
                        let _ = set_child_regs(pid, regs);

                        trap_addr = Some(possible_bp);
                        *last_breakpoint_addr = Some(possible_bp);
                        status_message = format!("Breakpoint hit at 0x{:08X}", possible_bp);
                        is_bp = true;
                    }
                } else if sig == Signal::SIGSEGV {
                    status_message =
                        format!("Segmentation fault (SIGSEGV) at RIP: 0x{:08X}", regs.rip);
                } else if sig == Signal::SIGILL {
                    status_message =
                        format!("Illegal instruction (SIGILL) at RIP: 0x{:08X}", regs.rip);
                } else if sig == Signal::SIGFPE {
                    status_message =
                        format!("Arithmetic exception (SIGFPE) at RIP: 0x{:08X}", regs.rip);
                }

                let _ = tx.send(DebuggerEvent::Stopped {
                    regs,
                    signal: sig,
                    trap_addr,
                    status_message,
                });

                if is_bp {
                    return WaitResult::BreakpointHit(trap_addr.unwrap());
                } else {
                    return WaitResult::StoppedSignal;
                }
            }
            Ok(WaitStatus::Exited(_, code)) => {
                let _ = tx.send(DebuggerEvent::Terminated {
                    exit_code: code,
                    status_message: format!("Process exited with status code {}", code),
                });
                return WaitResult::Exited;
            }
            Ok(WaitStatus::Signaled(_, sig, _)) => {
                let _ = tx.send(DebuggerEvent::Signaled {
                    signal: sig,
                    status_message: format!("Process killed by signal {:?}", sig),
                });
                return WaitResult::Exited;
            }
            other => {
                let _ = tx.send(DebuggerEvent::Error(format!(
                    "Unknown wait status: {:?}",
                    other
                )));
                return WaitResult::Exited;
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run_debugger_thread(_rx: Receiver<DebuggerCommand>, _tx: Sender<DebuggerEvent>) {
    // no-op stub on WASM targets
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_ptrace_flow() {
        // Map parent pages first
        let _ = setup_parent_mappings(); // Might be already mapped, ignore error

        let (cmd_tx, cmd_rx) = flume::unbounded();
        let (evt_tx, evt_rx) = flume::unbounded();

        // Spawn debugger thread
        let handle = std::thread::spawn(move || {
            run_debugger_thread(cmd_rx, evt_tx);
        });

        // Assemble a very simple code: "mov rax, 42; nop"
        // 48 c7 c0 2a 00 00 00 -> mov rax, 42
        // 90                   -> nop
        let code = vec![0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00, 0x90];

        // Start child with a breakpoint at CODE_BASE
        cmd_tx
            .send(DebuggerCommand::Start {
                code,
                initial_regs: CpuRegisters::default(),
                memory_patches: vec![],
                breakpoints: vec![CODE_BASE],
            })
            .unwrap();

        // Expect Started event
        match evt_rx.recv().unwrap() {
            DebuggerEvent::Started { regs, .. } => {
                assert_eq!(regs.rip, CODE_BASE as u64);
            }
            other => panic!("Expected Started event, got {:?}", other),
        }

        // Expect Stopped event due to the breakpoint on the first instruction
        match evt_rx.recv().unwrap() {
            DebuggerEvent::Stopped { regs, signal, .. } => {
                assert_eq!(signal, Signal::SIGTRAP);
                assert_eq!(regs.rip, CODE_BASE as u64);
            }
            other => panic!("Expected Stopped event, got {:?}", other),
        }

        // Step once: mov rax, 42
        cmd_tx.send(DebuggerCommand::Step).unwrap();
        match evt_rx.recv().unwrap() {
            DebuggerEvent::Stopped { regs, signal, .. } => {
                assert_eq!(signal, Signal::SIGTRAP);
                assert_eq!(regs.rax, 42);
            }
            other => panic!("Expected Stopped (SIGTRAP) event, got {:?}", other),
        }

        // Terminate
        cmd_tx.send(DebuggerCommand::Terminate).unwrap();
        match evt_rx.recv().unwrap() {
            DebuggerEvent::Terminated { .. } => {}
            other => panic!("Expected Terminated event, got {:?}", other),
        }

        // Clean up
        drop(cmd_tx);
        let _ = handle.join();
    }

    #[test]
    fn test_read_child_mem_boundary() {
        let _ = setup_parent_mappings();

        let (cmd_tx, cmd_rx) = flume::unbounded();
        let (evt_tx, evt_rx) = flume::unbounded();

        let handle = std::thread::spawn(move || {
            run_debugger_thread(cmd_rx, evt_tx);
        });

        let code = vec![0x90];

        cmd_tx
            .send(DebuggerCommand::Start {
                code,
                initial_regs: CpuRegisters::default(),
                memory_patches: vec![],
                breakpoints: vec![CODE_BASE],
            })
            .unwrap();

        let pid = match evt_rx.recv().unwrap() {
            DebuggerEvent::Started { pid, .. } => pid,
            other => panic!("Expected Started event, got {:?}", other),
        };

        match evt_rx.recv().unwrap() {
            DebuggerEvent::Stopped { .. } => {}
            other => panic!("Expected Stopped event, got {:?}", other),
        };

        let read_addr = STACK_BASE + STACK_SIZE - 8;
        let res = read_child_mem(pid, read_addr, 256);
        assert!(res.is_ok(), "Failed to read boundary: {:?}", res.err());
        let buf = res.unwrap();
        assert_eq!(buf.len(), 256);
        assert_eq!(&buf[8..256], &[0; 248]);

        // Terminate
        cmd_tx.send(DebuggerCommand::Terminate).unwrap();
        match evt_rx.recv().unwrap() {
            DebuggerEvent::Terminated { .. } => {}
            other => panic!("Expected Terminated event, got {:?}", other),
        }

        drop(cmd_tx);
        let _ = handle.join();
    }
}
