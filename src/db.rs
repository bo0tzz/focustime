use crate::butane_migrations;
use crate::models::Activity;
use butane::db::{Connection, ConnectionSpec};
use butane::migrations::Migrations;
use butane::{filter, DataObject, DataResult};

pub fn connect() -> Connection {
    let mut connection =
        butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap();
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
