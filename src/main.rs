mod workout;

fn main() {
    let mut workout = workout::Workout::new(600, workout::body_section::BodySection::Core);
    workout.new_settings("src/workout/settings.txt");
    workout.generate_excercises();
    println!("{:?}", workout.excercises.keys());
}






