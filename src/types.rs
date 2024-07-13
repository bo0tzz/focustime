use i3ipc::reply::{Node, WindowProperty};

pub struct FocusEvent {
    pub instance: String,
    pub title: String,
}

impl From<Node> for FocusEvent {
    fn from(value: Node) -> Self {
        let props = value.window_properties.unwrap();
        let instance = props.get(&WindowProperty::Instance).unwrap();
        let title = props.get(&WindowProperty::Title).unwrap();
        FocusEvent {
            instance: instance.clone(),
            title: title.clone(),
        }
    }
}
