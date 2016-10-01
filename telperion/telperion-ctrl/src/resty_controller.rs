extern crate iron;
extern crate router;

use self::iron::prelude::*;
use self::iron::headers::{Header, HeaderFormat, Headers, Location, ContentType};
use self::iron::{status, headers};

use self::router::Router;

pub struct RestyController {
    port: u16,
    iron: Option<Iron<Router>>,
    router: Box<Router>,
}

impl RestyController {
    pub fn new(port: u16) -> RestyController {
        let mut router: Box<Router> = Box::new(Router::new());

        return RestyController {
            port: port,
            iron: None,
            router: router,
        }
    }

    pub fn listen(mut self) {
        self.router.get("/im_ready",
                        |req: &mut Request| -> IronResult<Response> {
                            let res = (&self).nginx_ready(req);
                            return *res;
        //return Ok(Response::with((status::Ok, "OK.")));
                        });
        self.iron = Some(Iron::new(*(self.router)));

        //TODO: Use port member var and error handling.
        //TODO: Callback after server opens
        self.iron.unwrap().http("localhost:31337").unwrap();
    }

    fn nginx_ready(mut self, req: &mut Request) -> Box<IronResult<Response>>{
        return Box::new(Ok(Response::with((status::Ok, "OK."))));
    }
}
