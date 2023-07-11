//! Provides a plugin for Nickel's Request which allows the raw contents
//! of a body to be made available via the req.raw_body() method.

extern crate plugin;
extern crate typemap;

use self::plugin::{Pluggable, Plugin};
use self::typemap::Key;
use nickel::Request;
use std::io;
use std::io::Read;

struct RawBodyPlugin;

pub trait RawBody {
    fn raw_body(&mut self) -> &str;
}

impl Key for RawBodyPlugin {
    type Value = String;
}

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for RawBodyPlugin {
    type Error = io::Error;

    fn eval(req: &mut Request<D>) -> Result<String, io::Error> {
        let mut buffer = String::new();
        req.origin.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

impl<'mw, 'conn, D> RawBody for Request<'mw, 'conn, D> {
    fn raw_body(&mut self) -> &str {
        match self.get_ref::<RawBodyPlugin>().ok() {
            Some(x) => x,
            None => "",
        }
    }
}
