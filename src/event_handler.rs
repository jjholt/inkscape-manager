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
    pub fn listen_second_press(&'a self, current_style: &'a Style, target: Option<&str>) -> Style {
        let event = self.event_handler.connections.xcb.wait_for_event().unwrap();
        let new_style = if let xcb::Event::X(x::Event::KeyPress(ev)) = event {
            let new_key = ev.detail()
                .try_into_str()
                .and_then(|c| self.event_handler.keybinds.get_bind_for(c))
                .map(|c| KeyPress::new(ev, self.event_handler, &c.action));

            new_key
                .and_then(|f| f.send(target, true).unwrap())
                .and_then(|f| f.missing_param())
                .map(|f| current_style.set(f))
        } else {
            None
        };
        new_style.unwrap_or_else(|| current_style.to_owned())
    }
    pub fn send(self, target: Option<&str>, second_press: bool) -> xcb::Result<Option<&'a StyleList<'a>>> {
        match self.action {
            Action::Rebind { rebind_to: new_key } => {
                let new_key = new_key.try_into_keycode().unwrap();
                let cookie = self.event_handler.connections.xcb
                    .send_request_checked(&self.to_new_event(new_key, None).send());
                self.event_handler.connections.xcb.check_request(cookie)?;
                Ok(None)
            }
            Action::Style { style } => {
                // Check it's a filled style. If not, wait for a second keybind
                if second_press {
                    return Ok(Some(style))
                }

                let style_list: Vec<Style> = style
                    .iter()
                    .map(|c| { 
                        if c.missing_param() && !second_press {
                            self.listen_second_press(c, target)
                        } else { c.to_owned() } })
                    .collect();

                Svg::new(&StyleList(style_list))
                    .to_string()
                    .to_clipboard(target)
                    .unwrap();
                self.paste_style()?;
                Ok(None)
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
                    .map(|k| KeyPress::new(ev, self, k.action()));
                return Ok(keypress)
            }
            _ => (),
        };
        Ok(None)
    }
}
