use argparse::{ArgumentParser, Store, StoreTrue};

pub struct Input {
    pub allies: String,
    pub enemies: String,
    pub iterations: u64,
    pub threads: u64,
    pub print_version: bool,
    pub no_stats: bool,
    pub no_results: bool,
    pub bar: bool,
    pub manual_ally: bool,
    pub manual_enemy: bool,
    pub heroes_xml: String,
}

// Default for Input
impl Default for Input {
    fn default() -> Self {
        Self {
            allies: String::new(),
            enemies: String::new(),
            iterations: 1000,
            threads: 1,
            print_version: false,
            no_stats: false,
            no_results: false,
            bar: false,
            manual_ally: false,
            manual_enemy: false,
            heroes_xml: "data/heroes.xml".to_string(),
        }
    }
}

pub fn parse_args() -> Input {
    let mut input = Input::default();
    let mut no_gui = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Raid optimizer");
        ap.refer(&mut no_gui)
            .add_option(&["--no-gui"], StoreTrue, "No gui");
        ap.refer(&mut input.allies)
            .add_option(&["-a", "--ally"], Store, "Ally team")
            .required();
        ap.refer(&mut input.enemies)
            .add_option(&["-e", "--enemy"], Store, "Enemy team")
            .required();
        ap.refer(&mut input.iterations)
            .add_option(&["-i", "--iterations"], Store, "Number of iterations")
            .required();
        ap.refer(&mut input.threads)
            .add_option(&["-t", "--threads"], Store, "Number of threads");
        // print version
        ap.refer(&mut input.print_version).add_option(
            &["-v", "--version"],
            StoreTrue,
            "Print version",
        );
        ap.refer(&mut input.no_stats).add_option(
            &["--no-stats"],
            StoreTrue,
            "Don't print statistics",
        );
        ap.refer(&mut input.no_results).add_option(
            &["--no-results"],
            StoreTrue,
            "Don't print results",
        );
        ap.refer(&mut input.bar)
            .add_option(&["-b", "--bar"], StoreTrue, "Bar");
        ap.refer(&mut input.manual_ally)
            .add_option(&["--manual-ally"], StoreTrue, "Bar");
        ap.refer(&mut input.manual_enemy)
            .add_option(&["--manual-enemy"], StoreTrue, "Bar");
        ap.parse_args_or_exit();
    }

    input
}
