use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use crate::adapter::driven::infra::repositories::in_memory_user_repository::InMemoryUserRepository;
use crate::adapter::api::config::Config;
use crate::core::application::use_cases::user_use_case::UserUseCase;
use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;

use super::controllers::{user_controller, auth_controller};
use super::error_handling::generic_catchers;

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();
    let user_repository = Arc::new(Mutex::new(InMemoryUserRepository::new()));
    let user_use_case = UserUseCase::new(user_repository);

    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
        .merge(("port", 3000));

    rocket::build()
    .mount("/", routes![redirect_to_docs])
    .register("/", generic_catchers())
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            urls: vec![
                UrlObject::new("Auth", "/auth/openapi.json"),
                UrlObject::new("Users", "/users/openapi.json")
            ],
            ..Default::default()
        }),
    )
    .mount("/auth", auth_controller::routes())
    .mount("/users", user_controller::routes())
    .register("/users", user_controller::catchers())
    .manage(user_use_case)
    .configure(server_config)
    .launch()
    .await?;

    print!("Server running on {}", config.env.to_string());
    Ok(())
}