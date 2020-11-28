use hyper::Body;
use thruster::context::hyper_request::HyperRequest;
use thruster::context::typed_hyper_context::TypedHyperContext;
use thruster::hyper_server::HyperServer;
use thruster::App;
use thruster::ThrusterServer;
use thruster::{async_middleware, middleware_fn};
use thruster::{MiddlewareNext, MiddlewareResult};

use std::sync::Arc;
use std::sync::RwLock;

static ICON: &str = "data:image/x-icon;base64,AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAAAAAAgAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD//wAAznMAANWrAADb2wAA33sAAN47AADQCwAA3/sAANWrAADb2wAA1asAAN/7AADv9wAA9+8AAPgfAAD//wAA";

type Ctx = TypedHyperContext<RequestConfig>;

struct ServerConfig {
    val: Arc<RwLock<String>>,
}

struct RequestConfig {
    latest_value: Arc<RwLock<String>>,
}

fn generate_context(request: HyperRequest, state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(
        request,
        RequestConfig {
            latest_value: state.val.clone(),
        },
    )
}

#[middleware_fn]
async fn html(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
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
</html>"#
    .replace("{{ICON}}", ICON);
    context.body(&val);

    Ok(context)
}

#[middleware_fn]
async fn state_setter(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let req_body_result = context.get_body().await.expect("");
    let context = req_body_result.1;

    let latest_value = context.extra.latest_value.clone();
    let mut latest_value = latest_value.write().unwrap();
    *latest_value = req_body_result.0;

    Ok(context)
}

#[middleware_fn]
async fn state_getter(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let latest_value = context.extra.latest_value.clone();
    let latest_value = latest_value.read().unwrap();
    context.body = Body::from(format!("{}", latest_value));

    Ok(context)
}

fn main() {
    let mut app = App::<HyperRequest, Ctx, ServerConfig>::create(
        generate_context,
        ServerConfig {
            val: Arc::new(RwLock::new("UNSET".to_string())),
        },
    );
    app.get("/", async_middleware!(Ctx, [html]));
    app.post("/value", async_middleware!(Ctx, [state_setter]));
    app.get("/value", async_middleware!(Ctx, [state_getter]));

    let server = HyperServer::new(app);
    server.start("0.0.0.0", 4321);
}
