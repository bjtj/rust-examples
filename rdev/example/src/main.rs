use rdev::{listen, Event, EventType};

fn main() {
    println!("Hello, world!");
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    println!("Event {:?}", event);

    if let EventType::KeyPress(key) = event.event_type {
        println!("User pressed: {:?}", key);
    }
}
