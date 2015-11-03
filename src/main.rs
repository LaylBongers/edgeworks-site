extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "IT'S ALIIIIIVE!")))
    }

    Iron::new(hello_world).http("0.0.0.0:80").unwrap();
}
