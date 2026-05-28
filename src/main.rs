// Shellcide Assembler IDE main entry point. Code.
mod app;
mod assembler;
mod debugger;
mod disassembler;
mod editor;
mod encoder;
pub mod syntax_converter;
mod syscalls;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
use eframe::egui;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    // 1. Establish the memory segments in parent address space (inherited by fork)
    println!("[+] Initializing system memory mappings...");
    if let Err(err) = debugger::setup_parent_mappings() {
        eprintln!("[FATAL ERROR] Failed to map memory pages: {}", err);
        std::process::exit(1);
    }
    println!("[✓] Memory segments mapped successfully (Code, Data, Stack).");

    // 2. Setup message passing channels between GUI thread and Debugger thread
    let (cmd_tx, cmd_rx) = flume::unbounded();
    let (event_tx, event_rx) = flume::unbounded();

    // 3. Shared child Process ID for direct memory queries (/proc/pid/mem)
    let shared_pid = Arc::new(Mutex::new(None));

    // 4. Spawn background Debugger worker thread
    println!("[+] Spawning background debugger supervisor thread...");
    let debugger_event_tx = event_tx.clone();
    std::thread::spawn(move || {
        debugger::run_debugger_thread(cmd_rx, debugger_event_tx);
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
            Ok(Box::new(app::ShellcideApp::new(
                cc,
                cmd_tx,
                event_tx.clone(),
                event_rx,
                shared_pid,
            )))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect panic to browser console
    console_error_panic_hook::set_once();

    // Redirect log messages to browser console
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let (cmd_tx, _cmd_rx) = flume::unbounded();
        let (event_tx, event_rx) = flume::unbounded();
        let shared_pid = Arc::new(Mutex::new(None));

        use wasm_bindgen::JsCast;
        let canvas = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("the_canvas_id"))
            .expect("Failed to find canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Element is not a canvas");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    Ok(Box::new(app::ShellcideApp::new(
                        cc, cmd_tx, event_tx, event_rx, shared_pid,
                    )))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
