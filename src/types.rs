use i3ipc::reply::{Node, WindowProperty};

pub enum Focused {
    Nothing,
    Window { class: String, title: String },
}

impl From<Node> for Focused {
    fn from(value: Node) -> Self {
        let props = value.window_properties.unwrap();
        let class = props.get(&WindowProperty::Class).unwrap();
        let title = props.get(&WindowProperty::Title).unwrap();
        Focused::Window {
            class: class.clone(),
            title: title.clone(),
        }
    }
}
