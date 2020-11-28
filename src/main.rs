use thruster::{async_middleware, middleware_fn};
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareResult};

static ICON: &str = "data:image/x-icon;base64,AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAAAAAAgAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD//wAAznMAANWrAADb2wAA33sAAN47AADQCwAA3/sAANWrAADb2wAA1asAAN/7AADv9wAA9+8AAPgfAAD//wAA";

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = r#"<html>
<head>
    <title>ghosty</title>
    <link href="{{ICON}}" rel="icon" type="image/x-icon" />
    <style>img { image-rendering: pixelated; width: 64px; height: 64px; }</style>
</head>
<body>
    <img src="{{ICON}}" />
    <div>todo</div>
    <button disabled="true">copy</button>
</body>
</html>"#.replace("{{ICON}}", ICON);
    context.body(&val);
    Ok(context)
}

fn main() {
    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.get("/", async_middleware!(Ctx, [plaintext]));

    let server = Server::new(app);
    server.start("0.0.0.0", 4321);
}
