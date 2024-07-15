use i3ipc::reply::{Node, WindowProperty};

pub struct FocusedWindow {
    pub class: String,
    pub title: String,
}

pub enum Focused {
    Nothing,
    Window(FocusedWindow),
}

impl From<Node> for Focused {
    fn from(value: Node) -> Self {
        let props = value.window_properties.unwrap();
        let class = props.get(&WindowProperty::Class).unwrap();
        let title = props.get(&WindowProperty::Title).unwrap();
        Focused::Window {
            0: FocusedWindow {
                class: class.clone(),
                title: title.clone(),
            },
        }
    }
}
