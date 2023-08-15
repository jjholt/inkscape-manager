use xcb::x;
use crate::clipboard::Clipboard;
use crate::connection::*;
use crate::key::*;

use styler::keybind::*;

trait SendEvent {
    fn send(&self) -> x::SendEvent<Self> where Self: xcb::BaseEvent;
}

pub struct EventHandler<'a> {
    connections: &'a Connections<'a>,
    keybinds: &'a Keybinds<'a>,
    target: &'a Option<&'a str>,
}

pub struct KeyPress<'a,'b> {
    event: &'b x::KeyPressEvent,
    connections: &'a Connections<'a>,
    action: &'a Action<'a>,
    // target: &'a Option<&'a str>,
}

pub struct KeyPressResponse<'a> {
    connections: &'a Connections<'a>,
}

impl<'a> EventHandler<'a> {
    pub fn new(connections: &'a Connections<'a>, keybinds: &'a Keybinds<'a>, target: &'a Option<&'a str>) -> EventHandler<'a> {
       Self {connections, keybinds, target}
    }

    pub fn new_keypress<'b>(&self, event: &'b x::KeyPressEvent, action: &'a Action) -> KeyPress<'a,'b> {
        KeyPress {event, action, connections: self.connections}
    }

    pub fn listen(&self) -> xcb::Result<()> {
        let connections = self.connections;
        let xcb = connections.xcb;

        let event = xcb.wait_for_event()?;

        match event {
            xcb::Event::X(x::Event::PropertyNotify(_)) => {
                connections.get_active_window()?
                    .into_inkscape()
                    .and_then(|w| w.grab_key().ok());
            }
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                let keybind = ev.detail()
                    .try_into_str()
                    .and_then(|c| self.keybinds.get_bind_for(c)); 

                if let Some(keybind) = keybind {
                    // Send dbus command here
                    self
                        .new_keypress(&ev, keybind.action())
                        .send(self.target)
                        .flush()?;
                } 
            }
            _ => (),
        }
        Ok(())
    }
}

impl<'a, 'b> KeyPress<'a, 'b> {
    pub fn send(&self, target: &Option<&str>) -> KeyPressResponse<'_> {
        match self.action {
            Action::Rebind { rebind_to: new_key } => {
                let new_key = new_key.try_into_keycode().unwrap();
                self.connections.xcb.send_request_checked(&self.press(new_key).send());
                // self.connections.xcb.send_request_checked(&self.release(new_key).send());
            },
            Action::ApplyStyle(_style) => {
                let svg = crate::read(&std::path::PathBuf::from("style.svg")).unwrap();
                svg.to_clipboard(*target).unwrap();
                todo!();
                    // svg.paste(Some(target)).unwrap();
                    // Send ctrl + shift + v
            },
        }
        KeyPressResponse { connections: self.connections }
    }

    fn press(&self, new_key: x::Keycode) -> x::KeyPressEvent {
        let k = self.event;
        x::KeyPressEvent::new(
            new_key,
            k.time(), k.root(), k.event(),
            k.child(),k.root_x(),k.root_y(),
            k.event_x(),k.event_y(),k.state(),k.same_screen())
    }
}


impl SendEvent for x::KeyPressEvent {
    fn send(&self) -> x::SendEvent<Self> where Self: xcb::BaseEvent {
        x::SendEvent {
            propagate: true,
            // destination: x::SendEventDest::PointerWindow,
            destination: x::SendEventDest::Window(self.event()),
            event_mask: x::EventMask::KEY_PRESS | x::EventMask::KEY_RELEASE,
            event: self,
        } 
    }
}

impl<'a> KeyPressResponse<'a> {
    pub fn flush(&self) -> Result<(), xcb::ConnError> {
        self.connections.xcb.flush()
    }
}
