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

#[get("/11-cookies")]
pub fn _11() -> Template {
    let context = default_view_model();
    Template::render("docs/11-cookies", &context)
}

#[get("/12-users")]
pub fn _12() -> Template {
    let context = default_view_model();
    Template::render("docs/12-users", &context)
}

#[get("/13-multipart-upload")]
pub fn _13() -> Template {
    let context = default_view_model();
    Template::render("docs/13-multipart-upload", &context)
}

#[get("/14-logging")]
pub fn _14() -> Template {
    let context = default_view_model();
    Template::render("docs/14-logging", &context)
}

#[get("/15-email")]
pub fn _15() -> Template {
    let context = default_view_model();
    Template::render("docs/15-email", &context)
}

#[get("/16-constants")]
pub fn _16() -> Template {
    let context = default_view_model();
    Template::render("docs/16-constants", &context)
}

#[get("/17-sanitizer")]
pub fn _17() -> Template {
    let context = default_view_model();
    Template::render("docs/17-sanitizer", &context)
}

#[get("/18-error-handling")]
pub fn _18() -> Template {
    let context = default_view_model();
    Template::render("docs/18-error-handling", &context)
}

#[get("/19-assets")]
pub fn _19() -> Template {
    let context = default_view_model();
    Template::render("docs/19-assets", &context)
}

#[get("/20-deploying")]
pub fn _20() -> Template {
    let context = default_view_model();
    Template::render("docs/20-deploying", &context)
}

fn default_view_model() -> ViewModel<DefaultHeader, DefaultBody> {
    ViewModel::new(DefaultHeader::default(), DefaultBody::default())
}