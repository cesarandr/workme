mod workout;
mod arguments;


use arguments::Args;
use clap::Parser;
use std::str::FromStr;


fn main() {
    let args = Args::parse();
    let mut workout = workout::Workout::new(args.intensity, workout::body_section::BodySection::from_str(&args.section).unwrap());
    workout.new_settings("src/workout/settings.txt");
    workout.generate_excercises();
    workout.display_workout();
}






