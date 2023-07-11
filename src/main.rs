#[macro_use]
extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate argparse;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate derivative;

use argparse::{ArgumentParser, Store, StoreTrue};
use chrono::Utc;
pub mod message_handler;
pub mod messenger;
pub mod platform;
mod server;
pub mod telegram;
use server::{server as server_main, server::Server};
pub mod utils;

// use std::thread;
use tokio_threadpool::{ThreadPool, blocking};
use futures::Future;
use futures::future::{lazy, poll_fn};

#[tokio::main]
async fn main() {
    // -------------- Argument parser -------------------
    let mut address = "127.0.0.1:8080".to_string();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut verbose = false;
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut address)
            .add_option(&["--address"], Store, "Address to spin up the server");
        ap.parse_args_or_exit();
    }
    // -------------- start server -------------------
    let mut app = server_main::App::new();
    app.attach_endpoints();
    let mut telegram_api = telegram::api::TelegramAPI::new();
    let pool = ThreadPool::new();
    pool.spawn(lazy(move || {
        poll_fn(move || {
            blocking(|| {
                // let msg = rx.recv().unwrap();
                // println!("message = {}", msg);
            }).map_err(|_| panic!("the threadpool shut down"))
        })
    }));
    // handler.join().unwrap();
    app.start(address);
   
}

// use futures::StreamExt;
// use telegram_bot::*;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let token = dotenv!("TELEGRAM_TOKEN");
//     let api = Api::new(token);

//     // Fetch new updates via long poll method
//     let mut stream = api.stream();
//     while let Some(update) = stream.next().await {
//         // If the received update contains a new message...
//         let update = update?;
//         if let UpdateKind::Message(message) = update.kind {
//             if let MessageKind::Text { ref data, .. } = message.kind {
//                 // Print received text message to stdout.
//                 println!("<{}>: {}", &message.from.first_name, data);

//                 // Answer message with "Hi".
//                 api.send(message.text_reply(format!(
//                     "Hi, {}! You just wrote '{}'",
//                     &message.from.first_name, data
//                 )))
//                 .await?;
//             }
//         }
//     }
//     Ok(())
// }
