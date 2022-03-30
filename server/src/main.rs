use utils::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    println!("{:#?}", app)
}
