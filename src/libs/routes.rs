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
        pages_controller::google_verification,
        pages_controller::bing_verification,
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
        docs_controller::_04,
        docs_controller::_05,
        docs_controller::_06,
        docs_controller::_07,
        docs_controller::_08,
        docs_controller::_09,
        docs_controller::_10,
        docs_controller::_11,
        docs_controller::_12,
        docs_controller::_13,
        docs_controller::_14,
        docs_controller::_15,
        docs_controller::_16,
        docs_controller::_17,
        docs_controller::_18,
    ]
}
