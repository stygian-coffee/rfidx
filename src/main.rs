pub mod api;
pub mod app;
pub mod notify;

use app::App;

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let mut app = App::new();

    println!("{:?}", app.run().await);
}
