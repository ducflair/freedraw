mod app;

use app::App;

fn main() {
    // Start the Yew application
    yew::Renderer::<App>::new().render();
}
