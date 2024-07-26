mod config;
mod download;


fn main() {
    let cfg = config::init();
    println!("Using configuration: {:?}", cfg);
    let cards = download::download(cfg);
    println!("{:?}", cards);
}
