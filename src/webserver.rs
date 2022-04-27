use gotham_derive::StateData;

use gotham::{
    helpers::http::response::{create_response, create_empty_response},
    middleware::state::StateMiddleware,
    hyper::{Body, Response, StatusCode},
    pipeline::{single_pipeline, single_middleware},
    router::{builder::*, Router},
    state::{FromState, State}
};

use handlebars::Handlebars;
use std::{error::Error, sync::{Arc, Mutex}};
use log::{info, warn};

use crate::structure::Config;

#[derive(Clone, StateData)]
struct WebState {
    template: &'static str,
    state: Arc<Mutex<Config>>
}

fn generate_template(template: &str, state: Arc<Mutex<Config>>) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    let result = handlebars.register_template_file("template", template);

    match result {
        Ok(_msg) => info!("Template loaded successfully."),
        Err(_msg) => warn!("Error loading template")
    }

    let s = state.lock().unwrap();
    let rendered_template = handlebars.render("template", &*s)?;
    
    Ok(rendered_template)
}

fn response(state: State) -> (State, Response<Body>) {
    let web_state = WebState::borrow_from(&state);
    let template = generate_template(web_state.template, web_state.state.clone());

    let response = match template {
        Ok(rendered_template) => {
            create_response(&state, StatusCode::OK, "text/html".parse().unwrap(), rendered_template)
        },
        Err(_msg) => {
            create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    (state, response)
}

fn router(template: &'static str, static_path: &'static str, state: Arc<Mutex<Config>>) -> Router {

    let web_state = WebState { template: template, state: state};
    let middleware = StateMiddleware::new(web_state);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get("/").to(response);
        route.get("/static/*").to_dir(static_path);
    })
}

pub fn run(address: &'static str, template: &'static str, static_path: &'static str, state: Arc<Mutex<Config>>) {
    let result = gotham::start(address, router(template, static_path, state));

    match result {
        Ok(_msg) => info!("Webserver started successfully"),
        Err(_msg) => warn!("Error starting web server")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use gotham::hyper::StatusCode;

    fn generate_test_server() -> TestServer {
        let template_file = "./data/template.hbs";
        let static_path = "./data/static";
        let state: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
        let test_server = TestServer::new(router(template_file, static_path, state)).unwrap();

        return test_server;
    }

    #[test]
    fn index_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn static_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost/static/.gitkeep")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn static_test_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost/static/test.png")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn get_template() {
        let template_file = "./data/template.hbs";
        let state: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));

        let template = generate_template(template_file, state);

        let result =  match template {
            Ok(_ok) => 0,
            Err(_err) => 1
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn get_template_fail() {
        let template_file = "./data/fail.hbs";
        let state: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));

        let template = generate_template(template_file, state);

        let result =  match template {
            Ok(_ok) => 0,
            Err(_err) => 1
        };

        assert_eq!(result, 1);
    }
}