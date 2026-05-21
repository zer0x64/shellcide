// Shellcide Assembler IDE main entry point. Code.
mod assembler;
mod disassembler;
mod debugger;
mod editor;
mod app;
mod syscalls;
mod ui;

use std::sync::{Arc, Mutex};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // 1. Establish the memory segments in parent address space (inherited by fork)
    println!("[+] Initializing system memory mappings...");
    if let Err(err) = debugger::setup_parent_mappings() {
        eprintln!("[FATAL ERROR] Failed to map memory pages: {}", err);
        std::process::exit(1);
    }
    println!("[✓] Memory segments mapped successfully (Code, Data, Stack).");

    // 2. Setup message passing channels between GUI thread and Debugger thread
    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    
    // 3. Shared child Process ID for direct memory queries (/proc/pid/mem)
    let shared_pid = Arc::new(Mutex::new(None));
    let shared_pid_debug = shared_pid.clone();

    // 4. Spawn background Debugger worker thread
    println!("[+] Spawning background debugger supervisor thread...");
    std::thread::spawn(move || {
        debugger::run_debugger_thread(cmd_rx, event_tx);
    });

    // 5. Initialize Native GUI viewport window
    println!("[+] Starting eframe GUI loop...");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Shellcide // x86_64 Shellcoding IDE & Debugger")
            .with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Shellcide",
        options,
        Box::new(move |cc| {
            Ok(Box::new(app::ShellcideApp::new(cc, cmd_tx, event_rx, shared_pid_debug)))
        })
    )
}
