use enigo::{Button, Direction, Enigo, Settings, Mouse};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{self, Write};

fn main() {
    // Create an instance of Enigo with default settings
    let settings = Settings::default();
    let enigo_result = Enigo::new(&settings);

    let mut enigo = match enigo_result {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("Failed to initialize Enigo: {:?}", e);
            return;
        }
    };

    // Create an atomic flag to signal when to stop the loop
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    // Spawn a thread to listen for 'q' key press
    thread::spawn(move || {
        enable_raw_mode().expect("Failed to enable raw mode");
        //println!("Press 'q' to quit.");
        let mut stdout = io::stdout();
        stdout.execute(crossterm::cursor::Hide).unwrap();

        while r.load(Ordering::SeqCst) {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('q') {
                        r.store(false, Ordering::SeqCst);
                    }
                }
            }
        }

        disable_raw_mode().expect("Failed to disable raw mode");
        stdout.execute(crossterm::cursor::Show).unwrap();
    });

    // Main loop to simulate mouse clicks
    while running.load(Ordering::SeqCst) {
        // Simulate a mouse button press and release for a click
        if let Err(e) = enigo.button(Button::Left, Direction::Press) {
            eprintln!("Failed to press button: {:?}", e);
        }
        if let Err(e) = enigo.button(Button::Left, Direction::Release) {
            eprintln!("Failed to release button: {:?}", e);
        }

        //println!("Mouse click simulated at the current cursor position.");

        // Sleep for 1 second
        thread::sleep(Duration::from_secs(60));
    }

    println!("Program terminated.");
}
