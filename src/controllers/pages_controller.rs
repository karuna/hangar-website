use rocket_contrib::Template;
use view_models::*;

#[get("/")]
pub fn index() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("pages/index", &context)
}

#[get("/about")]
pub fn about() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("pages/about", &context)
}

#[get("/examples")]
pub fn examples() -> Template {
    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Template::render("pages/examples", &context)
}
