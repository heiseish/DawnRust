use super::messenger::*;
use super::nickel_rawbody::*;
use crate::info;
// use crate::telegram::api::TelegramAPI;
use nickel::status::StatusCode;
use nickel::{HttpRouter, MediaType, Nickel};

pub struct App {
    server: Nickel,
    // telegram_api: TelegramAPI,
}

pub trait Server {
    /// Set up endpoints
    fn attach_endpoints(&mut self);
    fn start(self, addr: String);
}
impl App {
    /// Static method to instantiate a new instance of app
    pub fn new() -> App {
        App {
            server: Nickel::new(),
            // telegram_api: TelegramAPI::new(),
        }
    }

    /// Attach facebook endpoint to the server
    /// In general, get request is to verify the server is legitimate and authenticated
    /// Post request handle the messenges from messenger
    fn load_facebook_endpoints(&mut self) {
        info!("Loading facebook messenger endpoints");
        self.server.get("/fb", load_facebook_endpoint_get);
        self.server.post("/fb", load_facebook_endpoint_post);
    }
    /// go to http://localhost:6767/raw to see this route in action
    fn load_samples_endpoint(&mut self) {
        info!("Loading samples endpoints");
        self.server.get(
            "/raw",
            middleware! { |_, mut res|
                res.set(MediaType::Json);
                r#"{ "foo": "bar" }"#
            },
        );

        self.server.post(
            "/raw",
            middleware! { |req, res|
                req.raw_body()
            },
        );
    }
    /// Load health check endpoint.
    ///
    /// For services like `heroku`, `/ping` endpoint
    /// can help to keep the server awake by calling the `/ping` endpoint
    /// in interval
    fn load_health_check_endpoints(&mut self) {
        info!("Loading health check endpoints");
        self.server.get("/", middleware!("dawn"));
        self.server.get(
            "/ping",
            middleware! { |_, mut res|
                res.set(StatusCode::Ok);
            },
        );
    }

    /// Turn on streaming connection like `twitter`, `codeforce` etc...
    fn load_streaming_services(&mut self) {
        info!("Loading streaming services");
    }
}


impl Server for App {
    /// Set up endpoints
    fn attach_endpoints(&mut self) {
        self.load_facebook_endpoints();
        self.load_samples_endpoint();
        self.load_health_check_endpoints();
        self.load_streaming_services();
    }

    fn start(self, addr: String) {
        info!("Server listening on {}", addr);
        self.server.listen(addr).unwrap();
    }
}
