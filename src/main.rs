mod api;
mod app;
mod file_index;
mod notify;

use app::App;

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let mut app = match App::new() {
        Ok(a) => a,
        Err(e) => {
            log::error!("Unable to start app: {}", e);
            return;
        }
    };

    app.run().await;
}
