use butane::db::Connection;
use butane::DataObject;
use chrono::Local;

use crate::models::{Event, EventType};
use crate::types::{Focused, FocusedWindow};

mod butane_migrations;
mod db;
mod i3;
mod models;
mod types;

fn main() {
    println!("Starting up focustime");

    let conn = db::connect();

    let handler = |event: Focused| log_event(&conn, event);

    i3::listen(handler);
}

fn log_event(conn: &Connection, event: Focused) {
    match event {
        Focused::Nothing => log_stop_event(conn),
        Focused::Window(e) => log_start_event(conn, e),
    }
}

fn log_stop_event(conn: &Connection) {
    let timestamp = Local::now().naive_local();
    let mut event = Event::new(EventType::Stop, timestamp, None);
    event.save(conn).unwrap();
    println!("{}: Stop", timestamp);
}

fn log_start_event(conn: &Connection, event: FocusedWindow) {
    let timestamp = Local::now().naive_local();
    let activity = db::get_or_create_activity(conn, &event.class, &event.title).unwrap();
    let mut event = Event::new(EventType::Start, timestamp, Some(activity.clone()));
    event.save(conn).unwrap();
    println!("{}: Start {}", timestamp, activity);
}
