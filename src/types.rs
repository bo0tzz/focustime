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
        match value.window_properties {
            Some(props) => {
                let class = props.get(&WindowProperty::Class).unwrap();
                let title = props.get(&WindowProperty::Title).unwrap();
                Focused::Window {
                    0: FocusedWindow {
                        class: class.clone(),
                        title: title.clone(),
                    },
                }
            }
            None => Focused::Nothing,
        }
    }
}
