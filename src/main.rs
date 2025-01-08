use rdev::{listen, Event, EventType};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Default)] // Add capabilities to the struct
struct DataFrame {
    m: i64,     // Minute since last epoch
    lc: i32,    // Left-click counter
    rc: i32,    // Right-click counter
    mc: i32,    // Middle-click counter
    ks: i32,    // Key-stroke counter
    mm: i32,    // Mouse-Movement counter
}

struct EventCounter {
    frame: DataFrame,
}
impl EventCounter {
    // Constructor
    fn new() -> Self {
        Self {
            frame: DataFrame::default(),
        }
    }

    // Event to update the DataFrame struct per input
    fn update_from_event(&mut self, event: &Event) {
        // Match here works similiar to a switch case where we run code per the event_type
        match event.event_type {
            // Handle mouse-button clicks
            EventType::ButtonPress(button) => {
                // Need to use 'match' to determine which type of mouse-button click
                match button {
                    rdev::Button::Left => {
                        self.frame.lc += 1;
                        println!("Left click: {}", self.frame.lc);
                    },
                    rdev::Button::Right => {
                        self.frame.rc += 1;
                        println!("Right click: {}", self.frame.rc);
                    },
                    rdev::Button::Middle => {
                        self.frame.mc += 1;
                        println!("Middle click: {}", self.frame.mc);
                    },
                    _ => {} // Handle any other mouse button types
                }
            },
            // Handle Key-strokes
            EventType::KeyPress(_) => {
                self.frame.ks += 1;
                println!("Key stroke: {}", self.frame.ks);
            },
            // Handle mouse movement
            EventType::MouseMove {..} => {
                self.frame.mm += 1;
                if self.frame.mm % 10 == 0 {
                    println!("Mouse movement: {}", self.frame.mm);
                }
            },
            // Ignore all other EventType(s)
            _ => {}
        }
    }
}



// Main program function
fn main() {
    println!("Starting Slime Watcher...");

    // Creating a thread-safe counter
    let counter = Arc::new(Mutex::new(EventCounter::new()));
    let counter_close = counter.clone(); // Clone for use in event listener

    // Setting the initial minute
    if let Ok(mut guard) = counter.lock() {
        guard.frame.m = chrono::Utc::now().timestamp() / 60;
    }

    // Starting the event listener
    if let Err(err) = listen(move |event| {
        if let Ok(mut guard) = counter_close.lock() {
            guard.update_from_event(&event);
        }
    }) {
        println!("error: {:?}", err)
    }
}