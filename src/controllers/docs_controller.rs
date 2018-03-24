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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "hangar new --name test_app --author \"Will Smoth <will_s@example.com>\" --database sqlite --database-url=test_app.sqlite
cd test_app".into());
    context.payload.insert(
        "02".to_string(),
        "fn root_urls() -> Vec<Route> {
    routes![
        ...
        pages_controller::hello_world
    ]
}"
            .into()
    );
    context.payload.insert(
        "03".to_string(),
        "<h1>Hello World!</h1>
<p>Welcome to Hangar application.</p>"
            .into()
    );
    Template::render("docs/03-hello_world", &context)
}

#[get("/04-configurations")]
pub fn _04() -> Template {
    let context = default_view_model();
    Template::render("docs/04-configurations", &context)
}

#[get("/05-routes")]
pub fn _05() -> Template {
    let mut context = default_view_model();
    context
        .payload
        .insert("01".to_string(), "Vec<Route>".into());
    context.payload.insert(
        "02".to_string(),
        "pub fn routes(attached_rocket: Rocket) -> Rocket {
    attached_rocket
        ...
        .mount(\"/posts\", post_urls())
}

fn post_urls() -> Vec<Route> {
    routes![
        posts_controller::new,
    ]
}"
            .into()
    );
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
    let mut context = default_view_model();
    context.payload.insert(
        "01".to_string(),
        "#[derive(Debug, Serialize)]
pub struct ViewModel<H, B>
where
    H: Serialize,
    B: Serialize,
{
    pub header: H,
    pub body: B,
    pub payload: Map<String, Value>,
}"
            .into()
    );
    context.payload.insert(
        "02".to_string(),
        "// src/controllers/example.rs

use rocket::response::Redirect;
use rocket_contrib::Template;
use view_models::*;

// You probably should put this struct into a module for better app structure
#[derive(Debug, Serialize)]
struct TestView {
    interesting_part: String,
}

#[get(\"/some_view\")]
pub fn some_view() -> Template {
    let test_view = TestView {
        interesting_part: String::from(\"ajep\")
    };
    let mut context = ViewModel::new(DefaultHeader::default(), &test_view);
    Template::render(\"some_view\", &context)
}

// src/views/some_view.html.tera
<h1>{{body.interesting_part}}</h1>"
            .into()
    );
    context.payload.insert(
        "03".to_string(),
        "// src/controllers/example.rs

use rocket::response::Redirect;
use rocket_contrib::Template;
use view_models::*;
use serde_json::Value;

#[get(\"/some_view\")]
pub fn some_view() -> Template {
    let mut context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    context.payload.insert(\"key\".to_string(), Value::Number(11.into()));
    Template::render(\"some_view\", &context)
}

// src/views/some_view.html.tera
<h1>{{payload.key}}</h1>"
            .into()
    );
    context.payload.insert(
        "04".to_string(),
        "{% extends \"base_layout\" %}

{% block content %}
  Insert some content here
{% endblock content %}"
            .into()
    );
    context.payload.insert(
        "05".to_string(),
        "<script src={{macros::asset_url(filename=\"application.js\")}}></script>".into()
    );
    context.payload.insert(
        "06".to_string(),
        "{% include \"path/to_some_file\" %}".into()
    );
    Template::render("docs/08-views", &context)
}

#[get("/09-models")]
pub fn _09() -> Template {
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "use chrono::NaiveDateTime;
use diesel::prelude::*;

use models::user::User;
use schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
#[table_name = \"posts\"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}".into());
    context.payload.insert("02".to_string(), "use validator::Validate;
...
#[derive(Validate, Insertable, Serialize)]
#[table_name = \"posts\"]
pub struct NewPost<'a> {
    pub user_id: i32,
    #[validate(length(min = \"1\"))]
    pub title: String,
    pub body: Option<&'a String>,
}".into());
    context.payload.insert("03".to_string(), "use rocket_contrib::Template;

use libs::db::ConnPool;
use models::post::Post;
use view_models::*;
use schema::posts;

#[get(\"/\", format = \"text/html\")]
pub fn index(db: ConnPool) -> Template {
    let posts_query = posts::dsl::posts.load::<Post>(&**db).optional();
    let posts_list = posts_query.unwrap();
    let header = DefaultHeader {
        title: String::from(\"Posts\"),
    };
    let context = ViewModel::new(&header, &posts_list);
    Template::render(\"posts/index\", &context)
}".into());
    context.payload.insert("04".to_string(), "use rocket::response::{Flash, Redirect};
...
#[get(\"/<id>\", format = \"text/html\")]
pub fn show(id: i32, db: ConnPool) -> Result<Template, Flash<Redirect>> {
    let post_query = posts::dsl::posts.find(&id).first::<Post>(&**db).optional();

    if let Ok(Some(post)) = post_query {
        let header = DefaultHeader {
            title: String::from(\"Post\"),
        };
        let context = ViewModel::new(&header, &post);
        Ok(Template::render(\"posts/show\", &context))
    } else {
        Err(Flash::error(Redirect::to(\"/posts\"), \"404\"))
    }
}".into());
    context.payload.insert("05".to_string(), "use std::default::Default;

use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use validator::Validate;

use rocket::request::LenientForm;
use guards::user::CurrentUser;
use models::post::{Post, PostFromParam};
...
#[post(\"/\", data = \"<post_param>\", format = \"application/x-www-form-urlencoded\")]
pub fn create(
    post_param: LenientForm<PostFromParam>,
    current_user: CurrentUser,
    db: ConnPool,
) -> Result<Flash<Redirect>, Template> {
    let new_post_from_param = post_param.into_inner();
    let user_id = current_user.id();
    let new_post = new_post_from_param.to_new_post(user_id);
    if new_post.validate().is_err() {
        return Err(render_new_post(current_user, Some(&new_post_from_param)));
    }
    let result = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(&**db);
    if let Err(Error::DatabaseError(_err, _)) = result {
        return Err(render_new_post(current_user, Some(&new_post_from_param)));
    }
    Ok(Flash::success(
        Redirect::to(\"/posts\"),
        \"Successfully created post.\",
    ))
}

fn render_new_post(user: CurrentUser, post: Option<&PostFromParam>) -> Template {
    let header = DefaultHeader {
        title: String::from(\"New Post\"),
    };
    if post.is_some() {
        let new_post = post.unwrap();
        let context = ViewModel::new(&header, &new_post);
        Template::render(\"posts/new\", &context)
    } else {
        let new_post = Post {
            user_id: user.id(),
            ..Default::default()
        };
        let context = ViewModel::new(&header, &new_post);
        Template::render(\"posts/new\", &context)
    }
}".into());
    context.payload.insert("06".to_string(), "use std::default::Default;

use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use validator::Validate;

use rocket::request::LenientForm;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use guards::user::CurrentUser;
use libs::db::ConnPool;
use models::post::{Post, PostFromParam};
use view_models::*;
use schema::posts;


#[post(\"/<id>\", data = \"<post_param>\", format = \"application/x-www-form-urlencoded\")]
pub fn update(
    id: i32,
    post_param: LenientForm<PostFromParam>,
    current_user: CurrentUser,
    db: ConnPool,
) -> Result<Flash<Redirect>, Template> {
    let post_from_param = post_param.into_inner();
    let user_id = current_user.id();
    let post_query = posts::dsl::posts.find(&id).first::<Post>(&**db).optional();
    if let Ok(Some(mut post)) = post_query {
        if user_id != post.user_id {
            return Ok(Flash::error(Redirect::to(\"/posts\"), \"Not authorized.\"));
        }
        post.title = post_from_param.title;
        post.body = post_from_param.body;
        let command = diesel::update(&post).set(&post);
        let result = command.get_result::<Post>(&**db).optional();
        if let Err(Error::DatabaseError(_err, _)) = result {
            return Err(render_edit_post(&post));
        }

        Ok(Flash::success(
            Redirect::to(\"/posts\"),
            \"Successfully created post\",
        ))
    } else {
        return Ok(Flash::error(Redirect::to(\"/posts\"), \"Not authorized.\"));
    }
}

fn render_edit_post(post: &Post) -> Template {
    let header = DefaultHeader {
        title: String::from(\"Edit Post\"),
    };
    let context = ViewModel::new(&header, &post);
    Template::render(\"posts/edit\", &context)
}".into());
    context.payload.insert("07".to_string(), "#[get(/<id>/delete, format = text/html)]
pub fn delete(
    id: i32,
    _current_user: CurrentUser,
    db: ConnPool,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let delete_query = posts::dsl::posts
        .find(&id)
        .select(posts::id)
        .first::<i32>(&**db)
        .optional();
    if let Ok(Some(_)) = delete_query {
        let result = diesel::delete(posts::dsl::posts.filter(posts::id.eq(&id)))
            .execute(&**db)
            .optional();
        if let Err(Error::DatabaseError(_err, _)) = result {
            return Ok(redirect_delete(Something went wrong.));
        }
        return Ok(redirect_delete(Successfully deleting post.));
    } else {
        return Ok(redirect_delete(Cannot find post.));
    }
}

fn redirect_delete(reason: &str) -> Flash<Redirect> {
    Flash::error(Redirect::to(\"/posts\"), reason)
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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "use rocket::Data;
use rocket::http::ContentType;
use multipart::server::Multipart;
use multipart::server::save::SaveResult::*;

#[post(\"/upload\", data = \"<data>\")]
pub fn upload(cont_type: &ContentType, data: Data) -> Result<Template, Template> {
    if !cont_type.is_form_data() {
        let body = json!({
            \"content\": \"Not a valid upload content\",
        });
        let context = ViewModel::new(DefaultHeader::default(), &body.0);
        return Err(Template::render(\"pages/index\", &context));
    }

    let (_, boundary) = cont_type
        .params()
        .find(|&(k, _)| k == \"boundary\")
        .ok_or_else(|| {
            let body = json!({
                \"content\": \"Not a valid upload content\",
            });
            let context = ViewModel::new(DefaultHeader::default(), &body.0);
            Template::render(\"pages/index\", &context)
        })?;

    let uploaded = process_upload(boundary, data);
    if uploaded.is_err() {
        let err = uploaded.unwrap();
        println!(\"Err\");
    } else {
        let result = uploaded.unwrap();
        println!(\"Ok\");
    }

    let context = ViewModel::new(DefaultHeader::default(), DefaultBody::default());
    Ok(Template::render(\"pages/index\", &context))
}

fn process_upload(boundary: &str, data: Data) -> Result<String, String> {
    Multipart::with_body(data.open(), boundary).foreach_entry(|mut entry|{
        let part = entry.data.save().size_limit(None).temp();
        match part {
            Full(full_part) => {
                // do something with full part
                println!(\"Full part {:?}\", full_part)
            },
            Partial(partial, reason) => {
                // do something with partial part
                println!(\"Request partially processed: {:?}\", reason);
                println!(\"{:?}\", partial);
            },
            Error(e) => println!(\"{:?}\", e),
        }

    });

    Ok(String::from(\"Success\"))
}".into());
    Template::render("docs/13-multipart-upload", &context)
}

#[get("/14-logging")]
pub fn _14() -> Template {
    let context = default_view_model();
    Template::render("docs/14-logging", &context)
}

#[get("/15-email")]
pub fn _15() -> Template {
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "use lettre::{EmailAddress, SimpleSendableEmail};
use libs::email::{Mailer, MailerConfig};

#[get(\"/email\")]
pub fn email(config: State<MailerConfig>) -> Template {
    let email = SimpleSendableEmail::new(
        EmailAddress::new(\"user@localhost\".to_string()),
        vec![EmailAddress::new(\"root@localhost\".to_string())],
        Uuid::new_v4().to_string(),
        \"Hello world\".to_string(),
    );

    let mailer_result = Mailer::new_from_config(&config.0);
    let send_mail = if mailer_result.is_ok() {
        mailer_result.unwrap().send_email(&email)
    } else {
        Err(String::from(\"Cannot send email\"))
    };
    let body = DefaultBody {
        content: send_mail.unwrap(),
    };
    let context = ViewModel::new(DefaultHeader::default(), &body);
    Template::render(\"pages/index\", &context)
}".into());
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
    let mut context = default_view_model();
    context.payload.insert("01".to_string(), "<img src=\"/assets/folder/image.jpg\" />".into());
    context.payload.insert("02".to_string(), "<script src={{macros::asset_url(filename=\"application.js\")}}></script>".into());
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
