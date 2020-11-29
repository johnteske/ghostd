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
use std::time::{Duration, Instant};

include!(concat!(env!("OUT_DIR"), "/html.rs"));

const TIMEOUT: Duration = Duration::from_secs(60);

type Ctx = TypedHyperContext<RequestConfig>;

struct ServerConfig {
    value: Arc<RwLock<String>>,
    expires: Arc<RwLock<Instant>>,
}

struct RequestConfig {
    value: Arc<RwLock<String>>,
    expires: Arc<RwLock<Instant>>,
}

fn generate_context(request: HyperRequest, state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(
        request,
        RequestConfig {
            value: state.value.clone(),
            expires: state.expires.clone(),
        },
    )
}

#[middleware_fn]
async fn check_expiration(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context = next(context).await?;

    let now = Instant::now();

    let expires = context.extra.expires.clone();
    let expires = expires.read().unwrap();

    // no time should have passed since expiration is hopefully in the future
    match now.checked_duration_since(*expires) {
        Some(_) => {
            println!("ERR");
            let value = context.extra.value.clone();
            let mut value = value.write().unwrap();
            *value = "UNSET".to_string();
        }
        None => {
            println!("OK");
        }
    }

    Ok(context)
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

    let value = context.extra.value.clone();
    let mut value = value.write().unwrap();
    *value = req_body_result.0;

    //context.body(&format!("{{\"message\": \"{}\",\"success\":false}}", e));
    context.body = Body::from(format!("{}", value));
    //context.body = Body::from(format!("{}, {}", value, expires.elapsed().as_secs_f32()));

    Ok(context)
}

#[middleware_fn]
async fn state_getter(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let value = context.extra.value.clone();
    let value = value.read().unwrap();

    context.body = Body::from(format!("{}", value));
    //context.body = Body::from(format!("{}, {}", value, expires.elapsed().as_secs_f32()));

    Ok(context)
}

fn main() {
    let mut app = App::<HyperRequest, Ctx, ServerConfig>::create(
        generate_context,
        // TODO share this with whatever reset fn I create
        ServerConfig {
            value: Arc::new(RwLock::new("UNSET".to_string())),
            expires: Arc::new(RwLock::new(Instant::now() + TIMEOUT)),
        },
    );
    app.get("/", async_middleware!(Ctx, [check_expiration, index]));
    app.post("/value", async_middleware!(Ctx, [state_setter]));
    app.get("/value", async_middleware!(Ctx, [state_getter]));

    let server = HyperServer::new(app);
    server.start("0.0.0.0", 4321);
}
