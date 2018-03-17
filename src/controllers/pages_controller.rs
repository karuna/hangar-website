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

#[get("/google36a72838f3e96989.html")]
pub fn google_verification() -> &'static str {
    "google-site-verification: google36a72838f3e96989.html"
}

#[get("/BingSiteAuth.xml")]
pub fn bing_verification() -> &'static str {
    "<?xml version=\"1.0\"?>
<users>
	<user>4E67BB81192FF0870E818D882DA362A5</user>
</users>"
}
