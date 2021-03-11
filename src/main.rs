pub mod app;

use app::App;

fn main() {
    let mut app = App::new();

    println!("{:?}", app.run());
}
