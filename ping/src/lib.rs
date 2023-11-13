#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};
use ping_io::*;

static mut MESSAGE_LOG: Vec<String> = vec![];

#[no_mangle]
extern "C" fn init() {
    let init_message: InitContractData = msg::load()
        .expect("Error in decoding InitContract");
    
    unsafe { MESSAGE_LOG = vec![] };
}

#[no_mangle]
extern fn handle() {
    let new_msg: RutzoAction = msg::load().expect("Unable to create string");

     unsafe { MESSAGE_LOG.push(String::from("ping")) };
    /*
    if new_msg == "PING" {
        msg::reply_bytes("PONG", 0).expect("Unable to reply");
    }

   

        debug!("{:?} total message(s) stored: ", MESSAGE_LOG.len());

        for log in &MESSAGE_LOG {
            debug!("{log:?}");
        }
    }
    */
}

/*
#[no_mangle]
extern fn state() {
    msg::reply(unsafe { MESSAGE_LOG.clone() }, 0)
        .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
}
*/

#[no_mangle]
extern fn state() {
    let state_message = msg::load()
        .expect("Error decoding 'PingStateQuery'");
    
    match state_message {
        RutzoStateQuery::GetText => {
            msg::reply(RutzoStateReply::Text(String::from("Texto!")), 0)
                .expect("Failed to encode or reply with <AppMetadata as Metadata>::State from 'state()'");
        },
        RutzoStateQuery::GetNumber => {
            msg::reply(RutzoStateReply::Number(64), 0)
                .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
        },
        RutzoStateQuery::All => {
            msg::reply(RutzoStateReply::All(unsafe { MESSAGE_LOG.clone() }), 0)
                .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gstd::{Encode, String};
    use gtest::{Log, Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current_opt(&system);

        let res = program.send_bytes(42, "INIT");
        assert!(!res.main_failed());

        let res = program.send_bytes(42, String::from("PING").encode());
        let log = Log::builder().source(1).dest(42).payload_bytes("PONG");
        assert!(res.contains(&log));
    }
}
