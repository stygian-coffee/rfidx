pub mod api;
pub mod app;

use app::App;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    println!("{:?}", app.run().await);
}
