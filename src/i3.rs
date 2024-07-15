use i3ipc::event::inner::{WindowChange, WorkspaceChange};
use i3ipc::event::{Event, WindowEventInfo, WorkspaceEventInfo};
use i3ipc::reply::{Node, NodeType};
use i3ipc::{I3EventListener, Subscription};

use crate::types::Focused;

pub fn listen(cb: impl Fn(Focused)) {
    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Window, Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    for event in listener.listen() {
        let focus_event = match event {
            Ok(Event::WindowEvent(e)) => process_window_focus(e),
            Ok(Event::WorkspaceEvent(e)) => process_workspace_focus(e),
            _ => unreachable!(),
        };

        if let Some(e) = focus_event {
            cb(e);
        }
    }
}

pub fn process_workspace_focus(e: WorkspaceEventInfo) -> Option<Focused> {
    if e.change != WorkspaceChange::Focus {
        return None;
    }
    match find_focused_node(e.current) {
        None => Some(Focused::Nothing),
        Some(node) if node.nodetype == NodeType::Workspace => return Some(Focused::Nothing),
        Some(node) => {
            return Some(Focused::from(node));
        }
    }
}

pub fn process_window_focus(e: WindowEventInfo) -> Option<Focused> {
    match e.change {
        WindowChange::Focus | WindowChange::Title => Some(Focused::from(e.container)),
        WindowChange::Close => Some(Focused::Nothing),
        _ => None,
    }
}

fn find_focused_node(node: Option<Node>) -> Option<Node> {
    if let Some(node) = node {
        if node.focused {
            return Some(node);
        }
        if let Some(&want) = node.focus.get(0) {
            let child = find_child(want, node.nodes.clone())
                .or_else(|| find_child(want, node.floating_nodes.clone()));

            return find_focused_node(child);
        }
    }
    return None;
}

fn find_child(want: i64, nodes: Vec<Node>) -> Option<Node> {
    for node in nodes {
        match node {
            node if node.id == want => return Some(node),
            _ => {}
        }
    }
    return None;
}
