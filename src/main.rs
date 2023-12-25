pub mod exceptions;
pub mod settings;
pub mod views {
    pub mod account;
    pub mod project;
}
pub mod utils {
    pub mod email;
    pub mod password;
    pub mod sequence;
    pub mod session;
}
pub mod handlers {
    pub mod account;
    pub mod project;
}

use oblivion::models::router::Router;
use oblivion::models::server::Server;
use oblivion::path_route;
use views::account::{login_handler, register_handler};
use crate::views::project::{filter_project_handler, finish_project_handler, new_project_handler, take_project_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut router = Router::new();

    path_route!(&mut router, "/login" => login_handler);
    path_route!(&mut router, "/register" => register_handler);
    path_route!(&mut router, "/project/new" => new_project_handler);
    path_route!(&mut router, "/project/take" => take_project_handler);
    path_route!(&mut router, "/project/filter" => filter_project_handler);
    path_route!(&mut router, "/project/finish" => finish_project_handler);
    let mut server = Server::new("127.0.0.1", 813, router);
    server.run().await;
}
