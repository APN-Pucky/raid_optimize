use raid_optimize::{input::parse_args, run};

#[tokio::main]
pub async fn main() {
    //env_logger::builder().filter_level(LevelFilter::Off).init();
    // check if env var is set
    let input = parse_args();
    run::runit(input).await;
}
