use rand::seq::SliceRandom;
use tokio::sync::mpsc;

use crate::data::load_heroes;
use super::{args::Args, results::CombinedResult};

//#[test]
//fn test_run() {
//    let mut rng = rand::thread_rng();
//    let heroes = load_heroes("data/heroes.xml".to_string());
//    let mut args = Args::default();
//    args.threads = 2;
//    args.iterations = 100;
//    // loop over all heroes and randomly select five per team
//    for _ in 0..10 {
//        args.allies = heroes.heroes.choose_multiple(&mut rng,5).cloned().collect();
//        args.enemies = heroes.heroes.choose_multiple(&mut rng,5).cloned().collect();
//        let sim = super::Sim::new(args.clone());
//        let (tx, mut rx) = mpsc::unbounded_channel::<CombinedResult>();
//        sim.run(tx);
//    }
//}