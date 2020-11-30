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

const TIMEOUT: Duration = Duration::from_secs(120);

type Ctx = TypedHyperContext<RequestConfig>;

struct ServerConfig {
    value: Arc<RwLock<String>>,
    expires: Arc<RwLock<Instant>>,
    seconds_until_exp: Arc<RwLock<u64>>,
}

struct RequestConfig {
    value: Arc<RwLock<String>>,
    expires: Arc<RwLock<Instant>>,
    seconds_until_exp: Arc<RwLock<u64>>,
}

fn generate_context(request: HyperRequest, state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(
        request,
        RequestConfig {
            value: state.value.clone(),
            expires: state.expires.clone(),
            seconds_until_exp: state.seconds_until_exp.clone(),
        },
    )
}

#[middleware_fn]
async fn check_expiration(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let now = Instant::now();

    let expires = context.extra.expires.clone();
    let expires = expires.read().unwrap();

    let duration_until_exp;
    match expires.checked_duration_since(now) {
        Some(d) => {
            println!("OK {}", d.as_secs());
            duration_until_exp = d;
        }
        None => {
            println!("ERR");
            let value = context.extra.value.clone();
            let mut value = value.write().unwrap();
            *value = "UNSET".to_string();

            duration_until_exp = Duration::new(0, 0);
        }
    }

    let seconds_until_exp = context.extra.seconds_until_exp.clone();
    let mut seconds_until_exp = seconds_until_exp.write().unwrap();
    *seconds_until_exp = duration_until_exp.as_secs();

    Ok(context)
}

#[middleware_fn]
async fn index(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.body(&HTML);

    Ok(context)
}

#[middleware_fn]
async fn state_setter(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let req_body_result = context
        .get_body()
        .await
        .expect("could not get request body");
    let mut context = req_body_result.1;

    let value = context.extra.value.clone();
    let mut value = value.write().unwrap();
    *value = req_body_result.0;

    let expires = context.extra.expires.clone();
    let mut expires = expires.write().unwrap();
    *expires = Instant::now() + TIMEOUT;

    let seconds_until_exp = context.extra.seconds_until_exp.clone();
    let mut seconds_until_exp = seconds_until_exp.write().unwrap();
    *seconds_until_exp = TIMEOUT.as_secs();

    context.body(&format!(
        "{{\"value\": \"{}\",\"seconds_until_exp\": \"{}\" }}",
        value, seconds_until_exp
    ));

    Ok(context)
}

#[middleware_fn]
async fn state_getter(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context = next(context).await?;

    let value = context.extra.value.clone();
    let value = value.read().unwrap();

    let seconds_until_exp = context.extra.seconds_until_exp.clone();
    let seconds_until_exp = seconds_until_exp.read().unwrap();

    context.body(&format!(
        "{{\"value\": \"{}\",\"seconds_until_exp\": \"{}\" }}",
        value, seconds_until_exp
    ));

    Ok(context)
}

fn main() {
    let mut app = App::<HyperRequest, Ctx, ServerConfig>::create(
        generate_context,
        ServerConfig {
            value: Arc::new(RwLock::new("UNSET".to_string())),
            expires: Arc::new(RwLock::new(Instant::now())),
            seconds_until_exp: Arc::new(RwLock::new(0)),
        },
    );
    app.get("/", async_middleware!(Ctx, [index]));
    app.post("/value", async_middleware!(Ctx, [state_setter]));
    app.get(
        "/value",
        async_middleware!(Ctx, [state_getter, check_expiration]),
    );

    let server = HyperServer::new(app);
    server.start("0.0.0.0", 4321);
}
