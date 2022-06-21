use rocket::put;
use rocket::{Build, Rocket};
use rocket_okapi::{
    mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec, swagger_ui::*,
};

#[rocket::main]
async fn main() {
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build().mount(
        "/swagger/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../v1/openapi.json".to_owned(),
            ..Default::default()
        }),
    );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
        "/issue" => openapi_get_routes_spec![openapi_settings: api_put_data],
    };

    return building_rocket;
}

/// # Put data
/// [Bug] format line `text/plain` is not working, openapi description always only includes `application/octet-stream`
#[openapi(tag = "Issue")]
#[put("/<key>", data = "<data>", format = "text/plain")]
fn api_put_data(key: String, data: String) -> Option<String> {
    Some(String::from(
        "key: ".to_string() + &key + "\ndata: " + &data,
    ))
}
