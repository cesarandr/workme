mod workout;
mod arguments;


use arguments::Arguments;
use arguments::Commands;
use clap::Parser;
use std::str::FromStr;


fn main() {
    let args = Arguments::parse();
    let mut workout: workout::Workout;
    let mut started_workout = false;

    match args.command {
        Some(Commands::Start { section, intensity }) => {
            workout = workout::Workout::new(intensity, workout::body_section::BodySection::from_str(&section).unwrap());
            workout.new_settings("src/workout/settings.txt");
            //workout.generate_excercises();
            workout.display_workout();
            workout.save();
        },
        Some(Commands::Stop) => {

        },
        Some(Commands::Account { name }) => println!("Account changed"),
        None => println!("Error")
    }
    


}






