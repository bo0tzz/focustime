use std::path::PathBuf;

use butane::db::{Connection, ConnectionSpec};
use butane::migrations::Migrations;
use butane::{filter, DataObject, DataResult};

use crate::butane_migrations;
use crate::models::Activity;

fn load_dev_config() -> Option<ConnectionSpec> {
    match ConnectionSpec::load(".butane/connection.json") {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn load_prod_config() -> ConnectionSpec {
    let home_dir = env!("HOME");

    let path: String = PathBuf::from_iter([home_dir, "focustime.db"])
        .into_os_string()
        .into_string()
        .unwrap();
    ConnectionSpec {
        backend_name: "sqlite".into(),
        conn_str: path,
    }
}

fn load_config() -> ConnectionSpec {
    load_dev_config().unwrap_or_else(load_prod_config)
}

pub fn connect() -> Connection {
    let mut connection = butane::db::connect(&load_config()).unwrap();
    let migrations = butane_migrations::get_migrations().unwrap();
    migrations.migrate(&mut connection).unwrap();
    connection
}

pub fn get_activity_by(
    conn: &Connection,
    class: &String,
    title: &String,
) -> butane::Result<Option<Activity>> {
    Activity::query()
        .filter(filter!(Activity, class == { class.clone() }))
        .filter(filter!(Activity, title == { title.clone() }))
        .load_first(conn)
}

pub fn get_or_create_activity(
    conn: &Connection,
    class: &String,
    title: &String,
) -> butane::Result<Activity> {
    match get_activity_by(conn, class, title) {
        Ok(Some(activity)) => Ok(activity),
        Ok(None) => {
            let mut activity = Activity::new(class, title);
            activity.save(conn).expect("Database insert failed");
            Ok(activity)
        }
        Err(e) => Err(e),
    }
}
