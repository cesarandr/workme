use clap::Parser;

/// A program wich generates a workout customized for your fitness level
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "workme")]
pub struct Args {
    #[clap(help = "choose wich body section to train", possible_values = ["upper", "core", "lower", "full"])]
    pub section: String,

    #[clap(help = "choose how hard your workout will be")]
    pub intensity: u32
}