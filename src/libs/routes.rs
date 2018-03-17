use rocket::{Rocket, Route};
use controllers::{docs_controller, pages_controller, users_controller};

pub fn routes(attached_rocket: Rocket) -> Rocket {
    attached_rocket
        .mount("/", root_urls())
        .mount("/users", user_urls())
        .mount("/docs", doc_urls())
}

fn root_urls() -> Vec<Route> {
    routes![
        pages_controller::index,
        pages_controller::about,
        pages_controller::examples,
    ]
}

fn user_urls() -> Vec<Route> {
    routes![
        users_controller::failed_show,
        users_controller::show,
        users_controller::signin,
        users_controller::signup,
        users_controller::login,
        users_controller::logout,
        users_controller::register,
    ]
}

fn doc_urls() -> Vec<Route> {
    routes![
        docs_controller::main,
        docs_controller::_01,
        docs_controller::_02,
        docs_controller::_03,
    ]
}
