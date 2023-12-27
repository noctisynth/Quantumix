pub mod exceptions;
pub mod settings;
pub mod views {
    pub mod account;
    pub mod project;
    pub mod todo;
}
pub mod utils {
    pub mod account;
    pub mod email;
    pub mod password;
    pub mod permission;
    pub mod sequence;
    pub mod session;
}
pub mod handlers {
    pub mod account;
    pub mod project;
    pub mod todo;
}

use crate::views::project::{
    filter_projects_handler, finish_project_handler, new_project_handler, take_project_handler,
};
use crate::views::todo::{
    filter_todos_handler, finish_todo_handler, new_todo_handler, take_todo_handler,
};
use oblivion::models::router::Router;
use oblivion::models::server::Server;
use oblivion::path_route;
use views::account::{login_handler, register_handler, session_handler};

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    path_route!(&mut router, "/login" => login_handler);
    path_route!(&mut router, "/register" => register_handler);
    path_route!(&mut router, "/session" => session_handler);
    path_route!(&mut router, "/todo/new" => new_todo_handler);
    path_route!(&mut router, "/todo/take" => take_todo_handler);
    path_route!(&mut router, "/todo/filter" => filter_todos_handler);
    path_route!(&mut router, "/todo/finish" => finish_todo_handler);
    path_route!(&mut router, "/project/new" => new_project_handler);
    path_route!(&mut router, "/project/take" => take_project_handler);
    path_route!(&mut router, "/project/filter" => filter_projects_handler);
    path_route!(&mut router, "/project/finish" => finish_project_handler);
    let mut server = Server::new("127.0.0.1", 813, router);
    server.run().await;
}
