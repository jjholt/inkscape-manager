pub use xcb::x;
pub use xcb_wm::{ewmh, icccm};

pub struct Connections<'a> {
    pub xcb: &'a xcb::Connection,
    pub ewmh: ewmh::Connection<'a>,
    pub icccm: icccm::Connection<'a>,
}

pub struct ActiveWindow<'a, 'b> {
    connections: &'b Connections<'a>,
    window: x::Window,
}

impl<'a> Connections<'a> {
    pub fn get_active_window(&self) -> xcb::Result<ActiveWindow> {
        let active_window_cookie = self.ewmh.send_request(&ewmh::GetActiveWindow);
        let reply = self.ewmh.wait_for_reply(active_window_cookie)?;
        Ok(ActiveWindow {
            connections: self,
            window: reply.window,
        })
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
            )],
        });
        xcb.check_request(cookie)?;

        Ok(Connections { xcb, ewmh, icccm })
    }
}

impl<'a, 'b> ActiveWindow<'a, 'b> {
    // pub fn window(&self) -> &x::Window {
    //     &self.window
    // }
    pub fn is_inkscape(&self) -> xcb::Result<bool> {
        let cookie = self.connections.icccm.send_request(&icccm::GetWmClass::new(self.window));
        let reply = self.connections.icccm.wait_for_reply(cookie)?;
        Ok(reply.class.to_lowercase().contains("inkscape"))
    }

    pub fn grab_key(&self) -> Result<(), xcb::ProtocolError> {
        println!("Now listening...");
        self.connections.xcb.send_and_check_request(&x::GrabKey {
            owner_events: true,
            grab_window: self.window,
            modifiers: x::ModMask::ANY,
            key: x::GRAB_ANY,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn ungrab_key(&self) -> Result<(), xcb::ProtocolError> {
        self.connections.xcb.send_and_check_request(&x::UngrabKey {
            key: x::GRAB_ANY,
            grab_window: self.window,
            modifiers: x::ModMask::ANY,
        })
    }
}
