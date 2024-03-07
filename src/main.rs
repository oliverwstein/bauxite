#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<h1>Welcome to My Rust Webpage</h1><p>This is the splash page.</p><a href=\"/about\">About Me</a>"
}

#[get("/about")]
fn about() -> &'static str {
    "<h1>About Me</h1><p>This is the About Me page.</p>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
}