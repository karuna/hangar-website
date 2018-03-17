use rocket::response::Redirect;
use rocket_contrib::Template;
use view_models::*;

#[get("/")]
pub fn main() -> Redirect {
    Redirect::to("/docs/01-what_you_need_to_prepare")
}

#[get("/01-what_you_need_to_prepare")]
pub fn _01() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("docs/01-what_you_need_to_prepare", &context)
}

#[get("/02-cli")]
pub fn _02() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("docs/02-cli", &context)
}

#[get("/03-hello_world")]
pub fn _03() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("docs/03-hello_world", &context)
}

#[get("/04-configurations")]
pub fn _04() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("docs/04-configurations", &context)
}