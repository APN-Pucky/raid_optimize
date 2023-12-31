use log::info;
use log::warn;

use quick_xml::de::from_str;
use tokio::sync::mpsc;

use crate::data::hero::get_hero_by_string;
use crate::data::heroes::Heroes;
use crate::debug;
use crate::input::Input;
use crate::sim;
use crate::sim::args::args_from_input;
use crate::sim::results::CombinedResult;
use crate::sim::Sim;

pub async fn runit(input: Input) {
    if input.print_version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let simargs = args_from_input(input);

    let mut sim: Sim = Sim::new(simargs);
    let (tx, mut rx) = mpsc::unbounded_channel::<CombinedResult>();
    sim.run(tx);
    let mut res = CombinedResult::default();
    while let Some(cr) = rx.recv().await {
        CombinedResult::add_combined_result(&mut res, &cr);
    }

    res.print();
}
