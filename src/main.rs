use thruster::{async_middleware, middleware_fn};
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareResult};

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

fn main() {
    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.get("/", async_middleware!(Ctx, [plaintext]));

    let server = Server::new(app);
    server.start("0.0.0.0", 4321);
}
