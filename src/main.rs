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

include!(concat!(env!("OUT_DIR"), "/html.rs"));

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
async fn index(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.body(&HTML);

    Ok(context)
}

#[middleware_fn]
async fn state_setter(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let req_body_result = context.get_body().await.expect("");
    let mut context = req_body_result.1;

    let latest_value = context.extra.latest_value.clone();
    let mut latest_value = latest_value.write().unwrap();
    *latest_value = req_body_result.0;
    context.body = Body::from(format!("{}", latest_value));

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
    app.get("/", async_middleware!(Ctx, [index]));
    app.post("/value", async_middleware!(Ctx, [state_setter]));
    app.get("/value", async_middleware!(Ctx, [state_getter]));

    let server = HyperServer::new(app);
    server.start("0.0.0.0", 4321);
}
