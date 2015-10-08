#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel};
use hyper::header::{Connection};

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, mut res| {
            res.headers_mut().set(Connection::close());
            "IT'S ALIIIIIIVE!"
        }
    });

    server.listen("0.0.0.0:80");
}
