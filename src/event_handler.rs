use styler::style::Style;
use styler::style::StyleList;
use xcb::x;

use crate::clipboard::Clipboard;
use crate::connection::*;
use crate::key::Code;
use crate::key::KeyTable;

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
    keycodes: KeyTable,
}

pub struct KeyPress<'a> {
    event_handler: &'a EventHandler<'a>,
    event: x::KeyPressEvent,
    action: &'a Action<'a>,
}

impl SendEvent for x::KeyPressEvent {}

impl <'a> KeyPress <'a> {
    pub fn new(event: x::KeyPressEvent, event_handler: &'a EventHandler, action: &'a Action,) -> Self where Self: Sized {
        Self {event, event_handler, action}
    }

    pub fn send(self, target: Option<&str>) -> Result<(), Box<dyn std::error::Error>>{
        match self.action {
            Action::Rebind { rebind_to: new_key } => {
                let new_key = self.event_handler.keycodes.keycode(new_key).expect("Trying to rebind unknown key");
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

    fn listen_second_press(&'a self, current_style: &'a Style) -> Style {
        // Instead of coupling the keypresses, listen for 0.5s ignoring key releases. So now the
        // keys can be pressed separately
        
        // let start_time = std::time::Instant::now();
        // while start_time.elapsed() <= std::time::Duration::from_millis(500) { 
        //     let this = self.event_handler.connections.xcb.wait_for_event().unwrap();
        //     if let xcb::Event::X(x::Event::KeyRelease(_)) = &this {
        //         break 
        //     }
        // }

        let event = self.event_handler.connections.xcb.wait_for_event().unwrap();
        let new_style: Option<Style> = if let xcb::Event::X(x::Event::KeyPress(ev)) = event {

            let new_key = self.event_handler.keycodes.code(ev.detail())
                .map(|f| f.to_string())
                .and_then(|c| self.event_handler.keybinds.get_bind_for(&c))
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

    fn get_style(self) -> Option<&'a StyleList<'a>> {
        match self.action {
            Action::Style { style } => Some(style),
            _ => None,
        }
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

    fn paste_style(&self) -> Result<(), xcb::ProtocolError> {
        let new_key = self.event_handler.keycodes.keycode("v").unwrap();
        // let new_key = "v".try_into_keycode().unwrap();
        let state = Some(x::KeyButMask::CONTROL | x::KeyButMask::SHIFT);
        let keypress = self.to_new_event(new_key, state);
        self.event_handler.connections.xcb.send_and_check_request(&keypress.send())
    }
}

impl<'a> EventHandler<'a> {
    pub fn new(connections: &'a Connections<'a>, keybinds: &'a Keybinds<'a>) -> EventHandler<'a> {
        Self {connections, keybinds, keycodes: KeyTable::new()}
    }

    pub fn listen(&self) -> xcb::Result<Option<KeyPress>> {
        let window = self.connections.get_active_window()?;

        let event = self.connections.xcb.wait_for_event()?;
        match event {
            xcb::Event::X(x::Event::PropertyNotify(_)) if window.is_inkscape()? => window.grab_key()?,
            xcb::Event::X(x::Event::PropertyNotify(_)) => window.ungrab_key()?,
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                let key = self.keycodes.code(ev.detail());

                if key == Some(Code::LeftSuper) {
                    println!("Keys are no longer being grabbed.");
                    window.ungrab_key()?;
                }

                let keypress = key
                    .map(|f| f.to_string())
                    .and_then(|f| self.keybinds.get_bind_for(&f))
                    .map(|k| KeyPress::new(ev, self, k.action()));
                return Ok(keypress)
            }
            _ => (),
        };
        Ok(None)
    }
}
