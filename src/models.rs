use std::fmt::{Display, Formatter};

use butane::{butane_type, model, AutoPk, FieldType, ForeignKey};
use chrono::NaiveDateTime;

#[model]
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: AutoPk<i64>,
    pub class: String,
    pub title: String,
}

impl Activity {
    pub fn new(class: impl Into<String>, title: impl Into<String>) -> Self {
        Activity {
            id: AutoPk::uninitialized(),
            class: class.into(),
            title: title.into(),
        }
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.class, self.title)
    }
}

#[butane_type(Text)]
#[derive(Debug, FieldType)]
pub enum EventType {
    Stop,
    Start,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            EventType::Stop => "Stop",
            EventType::Start => "Start",
        };
        write!(f, "{}", s)
    }
}

#[model]
#[derive(Debug)]
pub struct Event {
    pub id: AutoPk<i64>,
    pub event_type: EventType,
    pub timestamp: NaiveDateTime,
    pub activity: Option<ForeignKey<Activity>>,
}

impl Event {
    pub fn new(
        event_type: EventType,
        timestamp: NaiveDateTime,
        activity: Option<Activity>,
    ) -> Self {
        Event {
            id: AutoPk::uninitialized(),
            event_type,
            timestamp,
            activity: activity.map(|a| a.into()),
        }
    }
}
