#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<String> {
    RawHtml(format!(
        "<nav><a href=\"/\">Home</a> | <a href=\"/about\">About Me</a></nav>\
        <h1>Welcome to My Rust Webpage</h1>\
        <p>This is the splash page.</p>"
    ))
}

#[get("/about")]
fn about() -> RawHtml<String> {
    RawHtml(format!(
        "<nav><a href=\"/\">Home</a> | <a href=\"/about\">About Me</a></nav>\
        <h1>About Me</h1>\
        <p>This is the About Me page.</p>"
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
}
