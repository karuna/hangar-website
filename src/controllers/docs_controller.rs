use rocket::response::Redirect;
use rocket_contrib::Template;
use view_models::*;

#[get("/")]
pub fn main() -> Redirect {
    Redirect::to("/docs/01-what_you_need_to_prepare")
}

#[get("/01-what_you_need_to_prepare")]
pub fn _01() -> Template {
    let context = default_view_model();
    Template::render("docs/01-what_you_need_to_prepare", &context)
}

#[get("/02-cli")]
pub fn _02() -> Template {
    let context = default_view_model();
    Template::render("docs/02-cli", &context)
}

#[get("/03-hello_world")]
pub fn _03() -> Template {
    let context = default_view_model();
    Template::render("docs/03-hello_world", &context)
}

#[get("/04-configurations")]
pub fn _04() -> Template {
    let context = default_view_model();
    Template::render("docs/04-configurations", &context)
}

#[get("/05-routes")]
pub fn _05() -> Template {
    let context = default_view_model();
    Template::render("docs/05-routes", &context)
}

#[get("/06-controllers")]
pub fn _06() -> Template {
    let context = default_view_model();
    Template::render("docs/06-controllers", &context)
}

#[get("/07-actions")]
pub fn _07() -> Template {
    let context = default_view_model();
    Template::render("docs/07-actions", &context)
}

#[get("/08-views")]
pub fn _08() -> Template {
    let context = default_view_model();
    Template::render("docs/08-views", &context)
}

#[get("/09-models")]
pub fn _09() -> Template {
    let context = default_view_model();
    Template::render("docs/09-models", &context)
}

#[get("/10-middleware")]
pub fn _10() -> Template {
    let context = default_view_model();
    Template::render("docs/10-middleware", &context)
}

fn default_view_model() -> ViewModel<DefaultHeader, DefaultBody> {
    ViewModel::new(DefaultHeader::default(), DefaultBody::default())
}