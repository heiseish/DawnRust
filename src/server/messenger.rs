use crate::error;
use nickel::status::StatusCode;
use nickel::{JsonBody, MiddlewareResult, QueryString, Request, Response};

use crate::message_handler::{self, request};
use crate::messenger::request::*;

/// Facebook main post endpoint
///
pub fn load_facebook_endpoint_post<'mw>(
    _req: &mut Request,
    res: Response<'mw>,
) -> MiddlewareResult<'mw> {
    let msg = try_with!(res, {
        _req.json_as::<MessengerRequest>()
            .map_err(|e| (StatusCode::BadRequest, e))
    });
    let default = MessengerEvent::new();
    let events = msg.entry.first().unwrap_or(&default).get_events_ref();
    for event in events {
        let mut msg_request = request::Request::new();
        msg_request.sender_id = event.sender.id.clone();
        msg_request.platform = "messenger";
        if let Some(txt) = &event.message.text {
            msg_request.data = request::RequestType::Text(txt.clone());
        }
        if let Some(attachments) = &event.message.attachments {
            let attachment = attachments.first();
            if let Some(msg_attachment) = attachment {
                match &msg_attachment.type_name[..] {
                    "image" => {
                        msg_request.data =
                            request::RequestType::Image(String::new(), String::new());
                    }
                    "video" => {
                        msg_request.data =
                            request::RequestType::Image(String::new(), String::new());
                    }
                    "audio" => {
                        msg_request.data =
                            request::RequestType::Image(String::new(), String::new());
                    }
                    _ => {}
                }
            }
        }
        message_handler::handle_request(msg_request);
    }
    res.send(StatusCode::Ok)
}

/// Load facebook end point for server. FB_VERIFY compile time check for FB_verify token
///
/// Args:
///
/// try it with `curl -X GET "localhost:8080/fb?hub.verify_token=abc&hub.challenge=CHALLENGE_ACCEPTED&hub.mode=subscribe"`
pub fn load_facebook_endpoint_get<'mw>(
    _req: &mut Request,
    res: Response<'mw>,
) -> MiddlewareResult<'mw> {
    let req = _req.query();
    let mode = req.get("hub.mode").unwrap_or("");
    let verify_token = req.get("hub.verify_token").unwrap_or("");
    if mode == "subscribe" && verify_token == dotenv!("FB_VERIFY_TOKEN") {
        let response = req.get("hub.challenge").unwrap_or("");
        res.send(response)
    } else {
        error!(
            "Invalid request with mode {} and token {}",
            mode, verify_token
        );
        res.error(StatusCode::Forbidden, "Invalid request")
    }
}
