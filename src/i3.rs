use crate::types::FocusEvent;
use i3ipc::event::inner::{WindowChange, WorkspaceChange};
use i3ipc::event::{Event, WindowEventInfo, WorkspaceEventInfo};
use i3ipc::reply::Node;
use i3ipc::{I3EventListener, Subscription};

pub fn listen(cb: fn(FocusEvent)) {
    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Window, Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    for event in listener.listen() {
        let focus_event = match event {
            Ok(Event::WindowEvent(e)) => process_window_focus(e),
            Ok(Event::WorkspaceEvent(e)) => process_workspace_focus(e),
            _ => unreachable!(),
        };

        if focus_event.is_some() {
            cb(focus_event.unwrap());
        }
    }
}

pub fn process_workspace_focus(e: WorkspaceEventInfo) -> Option<FocusEvent> {
    if e.change != WorkspaceChange::Focus {
        return None;
    }
    match find_focused_node(e.current) {
        None => None,
        Some(node) => {
            return Some(FocusEvent::from(node));
        }
    }
}

pub fn process_window_focus(e: WindowEventInfo) -> Option<FocusEvent> {
    match e.change {
        WindowChange::Focus => Some(FocusEvent::from(e.container)),
        WindowChange::Title => Some(FocusEvent::from(e.container)),
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
