use actix::{Actor, Handler, Addr, Context, Message, ResponseActFuture, fut, ActorFuture, System};
//use futures::{future, Future};
use std::error::Error;
use futures::Future;

struct Request {
    pub msg: String,
}

struct Response {
    pub msg: String,
}

impl Message for Request {
    type Result = Result<Response, ()>;
}

#[derive(Debug)]
enum MyError {
    ParError,
}

impl Error for MyError {

}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParError")
    }
}

struct A;

impl Actor for A {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

    }
}

impl Handler<Request> for A {
    type Result = ResponseActFuture<Self, Response, ()>;

    fn handle(&mut self, req: Request, ctx: &mut Self::Context) -> Self::Result {
        Box::new(fut::ok(Response{msg: "it works".to_string()}))
    }
}

struct B {
    pub a: Addr<A>,
}

impl Actor for B {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

    }
}

impl Handler<Request> for B {
    type Result = ResponseActFuture<Self, Response, ()>;

    fn handle(&mut self, req: Request, ctx: &mut Self::Context) -> Self::Result {
        Box::new(fut::wrap_future(self.a.send(req))
            .map_err(|e,_,_| println!("Error {:?}", e))
            .and_then(|res,_,_| fut::result(res))
        )

        /*Box::new(fut::wrap_future(self.a.send(req))
            .map_err(|_, _, _| panic!("error"))
            .and_then(|res, _, _| fut::result(res)))*/
    }
}

#[test]
fn test() {
    let sys = System::builder().stop_on_panic(true).name("test").build();
    let a: Addr<A> = A.start();
    let b: Addr<B> = B{a}.start();
    b.send(Request{msg: "hello".to_string()});
}