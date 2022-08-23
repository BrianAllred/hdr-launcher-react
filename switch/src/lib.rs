use skyline_web::{Webpage, WebSession};
use std::thread::{self};
use serde::{Deserialize, Serialize};
use arcropolis_api::{self, ApiVersion, get_api_version};
use serde_json::Result;
use std::fmt;
use std::path::Path;

mod response;
mod message;
use response::*;
use message::*;

static HTML_TEXT: &str = include_str!("../web-build/index.html");
static JS_TEXT: &str = include_str!("../web-build/index.js");

#[skyline::main(name = "hdr-launcher-react")]
pub fn main() {
    println!("starting browser!");
    let browser_thread = thread::spawn(move || {
        let session = Webpage::new()
            .htdocs_dir("hdr-launcher")
            .file("index.html", &HTML_TEXT)
            .file("index.js", &JS_TEXT)
            .background(skyline_web::Background::Default)
            .boot_display(skyline_web::BootDisplay::Black)
            .open_session(skyline_web::Visibility::InitiallyHidden).unwrap();
        session.show();

        println!("session is open");
        listen_for_messages(&session);
    });

    // End thread so match can actually start
    browser_thread.join();
}

fn listen_for_messages(session: &WebSession) {
    loop {
        if let Some(msg) = session.try_recv() {
            println!("received a message: {}" , msg);
            let keep_listening = match serde_json::from_str::<Message>(&msg) {
                Ok(message) => message.handle(&session),
                Err(e) => {
                    println!("This is not a valid Message: {}", msg);
                    true
                }
            };
            
            // if the handling of one of our messages said to stop listening, then break.
            if !keep_listening {
                return;
            }
        }
    }
}

/// tries to open the main arcropolis configuration ui
pub fn try_open_arcropolis() {
    let exists = Path::new("sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libarcropolis.nro").exists();
    if !exists {
        println!("Error: We cannot open arcrop because you do not have arcropolis!");
        skyline_web::DialogOk::ok("Error: Cannot open arcropolis menu because you do not have arcropolis!");
        return;
    }
    let api_version = arcropolis_api::get_api_version();
    println!("opening arcrop menu...");
    if api_version.major >= 1 && api_version.minor >= 7 {
        arcropolis_api::show_main_menu();
    } else {
        println!("Error: We cannot open arcrop because arcrop is out of date!");
        skyline_web::DialogOk::ok("Error: Cannot open arcropolis menu because your arcropolis is out of date! You may want to update in the launcher.");
    }
}