use rocket::response::Redirect;
use rocket_contrib::Template;

use view_models::*;

#[get("/")]
pub fn main() -> Redirect {
    Redirect::to("/docs/01-preparation")
}

#[get("/01-preparation")]
pub fn _01() -> Template {
    let context = default_view_model();
    Template::render("docs/01-preparation", &context)
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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct AccessToken(String);

/// Returns true if `access_token` is valid.
fn is_valid(token: &str) -> bool {
    token == \"access_token\"
}

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AccessToken, ()> {
        let keys: Vec<_> = request.headers().get(\"x-access-token\").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if !is_valid(keys[0]) {
            return Outcome::Forward(());
        }

        return Outcome::Success(AccessToken(key.to_string()));
    }
}

#[get(\"/protected\")]
fn protected(key: AccessToken) -> &'static str {
    \"Authenticated.\"
}".into());
    Template::render("docs/07-actions", &context)
}

#[get("/08-views")]
pub fn _08() -> Template {
    let context = default_view_model();
    Template::render("docs/08-views", &context)
}

#[get("/09-models")]
pub fn _09() -> Template {
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "use validator::Validate;
...
#[derive(Validate, Insertable, Serialize)]
#[table_name = \"posts\"]
pub struct NewPost<'a> {
    pub user_id: i32,
    #[validate(length(min = \"1\"))]
    pub title: String,
    pub body: Option<&'a String>,
}".into());
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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "/users/<id>".into());
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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "#[derive(Debug, Serialize)]
struct TestView<'a> {
    content: &'a SanitizedStr,
}

#[get(\"/sanitized\")]
pub fn sanitized() -> Template {
    let header = DefaultHeader {
        title: String::from(\"Sanitized\"),
    };
    let test_str =
        SanitizedStr::new(\"<b><img src='' onerror='alert(\\'hax\\')'>I cannot hack you</b>\");
    let body = TestView { content: &test_str };
    let context = ViewModel::new(&header, &body);
    Template::render(\"pages/sanitize\", &context)
}".into());
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

#[get("/20-test")]
pub fn _20() -> Template {
    let context = default_view_model();
    Template::render("docs/20-test", &context)
}

#[get("/21-deploying")]
pub fn _21() -> Template {
    let context = default_view_model();
    Template::render("docs/21-deploying", &context)
}

fn default_view_model() -> ViewModel<DefaultHeader, DefaultBody> {
    ViewModel::new(DefaultHeader::default(), DefaultBody::default())
}
