use crate::types::FocusEvent;

mod i3;
mod types;

fn main() {
    println!("Starting up focustime");

    i3::listen(handle_current_focus);
}

fn handle_current_focus(event: FocusEvent) {
    println!("Current focus: {} - {}", event.instance, event.title);
}
