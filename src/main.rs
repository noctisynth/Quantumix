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

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    path_route!(&mut router, "/login" => login_handler);
    path_route!(&mut router, "/register" => register_handler);

    let mut server = Server::new("127.0.0.1", 813, router);
    server.run().await;
}
