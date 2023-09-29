// main.rs
use app::App;

mod app;
mod renderer;
mod input_handler;
mod flow;


fn main() {
    env_logger::init(); // Necessary for logging within WGPU
    let mut app = App::new();
    app.run();
}