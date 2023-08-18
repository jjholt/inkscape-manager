use xcb::x;

use crate::clipboard::Clipboard;
use crate::connection::*;
use crate::key::*;

use styler::keybind::*;

pub trait SendEvent {
    fn send(&self) -> x::SendEvent<Self> where Self: xcb::BaseEvent {
        x::SendEvent {
            propagate: true,
            destination: x::SendEventDest::PointerWindow,
            event_mask: x::EventMask::NO_EVENT,
            event: self,
        }
    }
}

pub struct EventHandler<'a> {
    connections: &'a Connections<'a>,
    keybinds: &'a Keybinds<'a>,
}

pub struct KeyPress<'a> {
    event: x::KeyPressEvent,
    connections: &'a Connections<'a>,
    action: &'a Action<'a>,
}

impl SendEvent for x::KeyPressEvent {}

impl <'a> KeyPress <'a>{
    pub fn send(&self, target: &Option<&str>) -> Result<(), xcb::ProtocolError> {
        match self.action {
            Action::Rebind { rebind_to: new_key } => {
                let new_key = new_key.try_into_keycode().unwrap();
                let cookie = self.connections.xcb
                    .send_request_checked(&self.to_new_event(new_key, None).send());
                self.connections.xcb.check_request(cookie)?;
                Ok(())
            }
            Action::ApplyStyle(_style) => {
                let svg = crate::read(&std::path::PathBuf::from("examples/dashed.svg")).unwrap();
                svg.to_clipboard(*target).unwrap();
                // self.connections.get_active_window().unwrap().ungrab_key().unwrap();
                self.paste_style()?;
                Ok(())
                    // svg.paste(Some(target)).unwrap();
                    // Send ctrl + shift + v
            }
        }
    }

    fn to_new_event(&self, new_key: x::Keycode, state: Option<x::KeyButMask>) -> x::KeyPressEvent {
        let ev = &self.event;
        let state = state.unwrap_or(ev.state());
        x::KeyPressEvent::new(
            new_key,
            ev.time(), ev.root(), ev.event(), ev.child(),
            ev.root_x(), ev.root_y(), ev.event_x(), ev.event_y(),
            state, 
            ev.same_screen(),
        )
    }

    fn new(event: x::KeyPressEvent, connections: &'a Connections, action: &'a Action,) -> Self where Self: Sized {
        Self {event, connections, action}
    }

    fn paste_style(&self) -> Result<(), xcb::ProtocolError> {
        let new_key = "v".try_into_keycode().unwrap();
        let state = Some(x::KeyButMask::CONTROL | x::KeyButMask::SHIFT);
        let keypress = self.to_new_event(new_key, state);
        self.connections.xcb.send_and_check_request(&keypress.send())
    }
}

impl<'a> EventHandler<'a> {
    pub fn new( connections: &'a Connections<'a>, keybinds: &'a Keybinds<'a>) -> EventHandler<'a> {
        Self {connections, keybinds}
    }

    pub fn listen(&self) -> xcb::Result<Option<KeyPress>> {
        let connections = self.connections;
        let xcb = connections.xcb;

        let event = xcb.wait_for_event()?;

        let window = connections.get_active_window()?;
        match event {
            xcb::Event::X(x::Event::PropertyNotify(_)) if window.is_inkscape()? => window.grab_key()?,
            xcb::Event::X(x::Event::PropertyNotify(_)) => window.ungrab_key()?,
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                if ev.detail() == 134 {
                    //Win key
                    println!("Keys are no longer being grabbed.");
                    window.ungrab_key()?;
                }

                let keypress = ev
                    .detail()
                    .try_into_str()
                    .and_then(|c| self.keybinds.get_bind_for(c))
                    .map(|k| KeyPress::new(ev, connections, k.action()));
                return Ok(keypress)
            }
            _ => (),
        };
        Ok(None)
    }
}
