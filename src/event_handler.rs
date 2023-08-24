use styler::style::Style;
use styler::style::StyleList;
use xcb::x;

use crate::clipboard::Clipboard;
use crate::connection::*;
use crate::key::*;

use styler::keybind::*;
use styler::Svg;

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
    event_handler: &'a EventHandler<'a>,
    event: x::KeyPressEvent,
    action: &'a Action<'a>,
}

impl SendEvent for x::KeyPressEvent {}

impl <'a> KeyPress <'a> {
    pub fn listen_second_press(&'a self, current_style: &'a Style) -> Style {
        let event = self.event_handler.connections.xcb.wait_for_event().unwrap();
        let new_style = if let xcb::Event::X(x::Event::KeyPress(ev)) = event {
            let new_key = ev.detail()
                .try_into_str()
                .and_then(|c| self.event_handler.keybinds.get_bind_for(c))
                .map(|c| KeyPress::new(ev, self.event_handler, &c.action));

            new_key
                .and_then(|f| f.get_style())
                .and_then(|f| f.missing_param())
                .map(|f| current_style.set(f))
        } else {
            None
        };
        new_style.unwrap_or_else(|| current_style.to_owned())
    }

    pub fn get_style(self) -> Option<&'a StyleList<'a>> {
       match self.action {
        Action::Style { style } => Some(style),
        _ => None,
    }
    }
    pub fn send(self, target: Option<&str>) -> Result<(), Box<dyn std::error::Error>>{
        match self.action {
            Action::Rebind { rebind_to: new_key } => {
                let new_key = new_key.try_into_keycode().expect("Trying to rebind to unknown key");
                let cookie = self.event_handler.connections.xcb
                    .send_request_checked(&self.to_new_event(new_key, None).send());
                self.event_handler.connections.xcb.check_request(cookie)?;
            }
            Action::Style { style } => {
                let style_list: Vec<Style>= style
                    .iter()
                    .map(|c| c.missing_param()
                         .then(|| self.listen_second_press(c))
                         .unwrap_or_else(|| c.to_owned()) )
                    .collect();

                Svg::new(&style_list.into())
                    .to_string()
                    .to_clipboard(target)?;

                self.paste_style()?;
            }
        }
        Ok(())
    }

    fn to_new_event(&self, new_key: x::Keycode, state: Option<x::KeyButMask>) -> x::KeyPressEvent {
        let ev = &self.event;
        x::KeyPressEvent::new(
            new_key,
            ev.time(), ev.root(), ev.event(), ev.child(),
            ev.root_x(), ev.root_y(), ev.event_x(), ev.event_y(),
            state.unwrap_or(ev.state()), 
            ev.same_screen(),
        )
    }

    fn new(event: x::KeyPressEvent, event_handler: &'a EventHandler, action: &'a Action,) -> Self where Self: Sized {
        Self {event, event_handler, action}
    }

    fn paste_style(&self) -> Result<(), xcb::ProtocolError> {
        let new_key = "v".try_into_keycode().unwrap();
        let state = Some(x::KeyButMask::CONTROL | x::KeyButMask::SHIFT);
        let keypress = self.to_new_event(new_key, state);
        self.event_handler.connections.xcb.send_and_check_request(&keypress.send())
    }
}

impl<'a> EventHandler<'a> {
    pub fn new(connections: &'a Connections<'a>, keybinds: &'a Keybinds<'a>) -> EventHandler<'a> {
        Self {connections, keybinds}
    }

    pub fn listen(&self) -> xcb::Result<Option<KeyPress>> {
        let window = self.connections.get_active_window()?;

        let event = self.connections.xcb.wait_for_event()?;
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
                    .map(|k| KeyPress::new(ev, self, k.action()));
                return Ok(keypress)
            }
            _ => (),
        };
        Ok(None)
    }
}
