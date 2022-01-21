use clap::{self, Parser, Args, Subcommand, ArgEnum};

/// A program wich generates a workout customized for your fitness level
/// It tracks your progress and lets you monitor your progression
#[derive(Parser)]
#[clap(author, version, about, long_about = None, name = "workme")]
pub struct Arguments {

    #[clap(subcommand)]
    pub command: Option<Commands>,

}

#[derive(Subcommand)]
pub enum Commands {
    /// change your account
    Account {
        #[clap(help = "choose your account")]
        name: String
    },
    /// start your workout
    Start {
        #[clap(help = "choose a body section to train", possible_values = ["upper", "core", "lower", "full"])]
        section: String,
        #[clap(help = "choose the intensity of the workout")]
        intensity: u32
    },
    /// stop a started workout
    Stop

}


