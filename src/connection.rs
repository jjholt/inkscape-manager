pub use xcb_wm::{ewmh, icccm};
pub use xcb::x;

pub struct Connections<'a> {
    pub xcb: &'a xcb::Connection,
    pub ewmh: ewmh::Connection<'a>,
    pub icccm: icccm::Connection<'a>,
}

pub struct ActiveWindow<'a, 'b> {
    connections: &'b Connections<'a>,
    window: x::Window,
}

impl <'a> Connections<'a> {
    pub fn get_active_window(&self) -> xcb::Result<ActiveWindow> {
        let active_window_cookie = self.ewmh.send_request(&ewmh::GetActiveWindow);
        let reply = self.ewmh.wait_for_reply(active_window_cookie)?;
        Ok(ActiveWindow {connections: self, window:  reply.window})
    }
    pub fn setup(xcb: &'a xcb::Connection, screen_num: i32) -> xcb::Result<Connections<'a>> {
        // Create connections and find root window
        let setup = xcb.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        let ewmh = ewmh::Connection::connect(xcb);
        let icccm = icccm::Connection::connect(xcb);
        
        // Make the root window issue events on property changes. I.e. it knows something has been
        // changed when you mouse-over to another window.
        let cookie = xcb.send_request_checked(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[x::Cw::EventMask(
                x::EventMask::PROPERTY_CHANGE 
                | x::EventMask::KEY_RELEASE
                | x::EventMask::KEY_PRESS 
            )],
        });
        xcb.check_request(cookie)?;

        let cookie = ewmh.send_request(&ewmh::GetClientList);
        let windows = ewmh.wait_for_reply(cookie)?.clients;
        for window in windows {
            let request = &icccm::GetWmClass::new(window);
            let cookie = icccm.send_request(request);
            let class = icccm.wait_for_reply(cookie)?.class;
            if class.to_lowercase().contains("inkscape") {
                println!("Found inkscape: {:#?}", window);
            }
        }
        Ok(Connections {xcb, ewmh, icccm})
    }
}


impl <'a, 'b> ActiveWindow<'a, 'b> {
    // pub fn window(&self) -> &x::Window {
    //     &self.window
    // }
    pub fn into_inkscape(self) -> Option<ActiveWindow<'a,'b>> {
        let request_class = icccm::GetWmClass::new(self.window);
        let cookie = self.connections.icccm.send_request(&request_class);
        let reply = self.connections.icccm.wait_for_reply(cookie).ok();
        match reply {
            Some(v) if v.class.to_lowercase().contains("inkscape") => Some(ActiveWindow {..self}),
            _ => None ,
        }
    }

    pub fn grab_key(&self) -> Result<(), xcb::ProtocolError> {
        let cookie = self.connections.xcb.send_request_checked(&x::GrabKey {
            owner_events: false,
            grab_window: self.window,
            modifiers: x::ModMask::ANY,
            key: x::GRAB_ANY,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        });
        println!("Now listening...", );
        self.connections.xcb.check_request(cookie)
    }

    pub fn ungrab_key(&self) -> Result<(), xcb::ProtocolError> {
        let cookie = self.connections.xcb.send_request_checked(&x::UngrabKey {
            key: x::GRAB_ANY,
            grab_window: self.window,
            modifiers: x::ModMask::ANY,
        });
        self.connections.xcb.check_request(cookie)
    }
}
