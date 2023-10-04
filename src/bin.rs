extern crate argparse;











use raid_optimize::ui;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    //let args: Vec<String> = env::args().collect();
    //if let 2 = args.len() {
    //    if let Ok("--no-gui") = args[1].parse() {
    //        env_logger::init();
    //        // exit 0
    //        return;
    //    }
    //}
    ui::main().await;

    //let mut input = Input::parse_args();
    //run(input);
   
}