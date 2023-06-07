use std::error::Error;

use xcb::x;

pub fn connect(event_handler: &dyn Fn(Option<char>) -> Result<(), Box<dyn Error>>) -> xcb::Result<()> {
    let (conn,_) = xcb::Connection::connect(None)?;
    // let setup = conn.get_setup();

    println!("Done setup");
    loop {
        let event = match conn.wait_for_event() {
            Err(xcb::Error::Connection(err)) => panic!("unexpected I/O error: {}", err),
            // Err(xcb::Error::Protocol(xcb::ProtocolError::X(x::Error::Font(err), _req_name))) => continue,
            Err(xcb::Error::Protocol(err)) => panic!("unexpected protocol error: {:#?}", err),
            Ok(event) => event,
        };
        println!("Now starting to match event");
        match event {
            xcb::Event::X(x::Event::KeyPress(event)) => {
                println!("{:#?}", event);
                
                // event_handler(key);
            }
            _ => break Ok(())
        }
    }
}
