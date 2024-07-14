use crate::types::Focused;

mod i3;
mod types;

fn main() {
    println!("Starting up focustime");

    i3::listen(handle_current_focus);
}

fn handle_current_focus(event: Focused) {
    match event {
        Focused::Nothing => println!("Nothing focused"),
        Focused::Window { class, title } => println!("Current focus: {} - {}", class, title),
    }
}
