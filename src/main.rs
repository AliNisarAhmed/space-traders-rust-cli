#[tokio::main]
async fn main() {
    let args = space_traders_rust::get_args().unwrap();
    if let Err(e) = space_traders_rust::run(args).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
