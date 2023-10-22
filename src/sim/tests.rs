use rand::seq::SliceRandom;
use tokio::sync::mpsc;

use super::{args::Args, results::CombinedResult};
use crate::data::load_heroes;

macro_rules! test_run_N_vs_M {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut rng = rand::thread_rng();
                let heroes = load_heroes("data/heroes.xml".to_string());
                let mut args = Args::default();
                args.threads = 2;
                args.iterations = 100;
                // loop over all heroes and randomly select five per team
                for _ in 0..25 {
                    args.allies = heroes
                        .heroes
                        .choose_multiple(&mut rng, $value.0)
                        .cloned()
                        .collect();
                    args.enemies = heroes
                        .heroes
                        .choose_multiple(&mut rng, $value.1)
                        .cloned()
                        .collect();
                    let sim = super::Sim::new(args.clone());
                    let (tx, _rx) = mpsc::unbounded_channel::<CombinedResult>();
                    sim.run(tx);
                }
            }
        )*
        }
}

test_run_N_vs_M! {
    test_run_1_vs_1: (1, 1),
    test_run_2_vs_1: (2, 1),
    test_run_1_vs_2: (1, 2),
    test_run_2_vs_2: (2, 2),
    test_run_3_vs_3: (3, 3),
    test_run_4_vs_4: (4, 4),
    test_run_5_vs_5: (5, 5),
    test_run_1_vs_5: (1, 5),
    test_run_5_vs_1: (5, 1),
}
