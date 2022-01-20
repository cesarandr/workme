mod exercise;
pub mod body_section;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use rand::Rng;
use std::time::SystemTime;
use sqlite::{ self, Value };

// Workout definition
pub struct Workout {
    pub intensity: u32,
    pub section: body_section::BodySection,
    pub duration: u32,
    pub excercises: HashMap<String, exercise::Exercise>,
    pub breaks: f32,
    settings: HashMap<String, String>,
    statistics: HashMap<String, String>
}

impl Workout {
    pub fn new(intensity: u32, section: body_section::BodySection) -> Workout {
        let mut settings = HashMap::new();
        settings.insert(String::from("maximum exercises"), String::from("10"));
        settings.insert(String::from("maximum repetitions"), String::from("50"));

        let mut statistics = HashMap::new();
        statistics.insert(String::from("complete intensity"), String::from("0"));

        Workout { intensity, section, duration: 0, excercises: HashMap::new() , breaks: 4.0, settings, statistics }
    }

    pub fn generate_excercises(&mut self) {
        // get the limit from the settings
        let max_exercises: u32 = self.settings.get(&String::from("maximum exercises")).expect("maximum exercises setting wasn't set").parse().unwrap();
        // stats
        let mut max_intensity = 0;
        let mut full_duration = 0;
        let mut generated_exercises = 0;

        let mut exercise_group = exercise::Exercise::get_group(self.section.clone()).into_iter(); // get all excercises for a specific body section
        let mut rng = rand::thread_rng();
        let number_of_exercises = rng.gen_range(5..exercise_group.len()); // decide randomly how many exercises the workout will include

        // algorithm to fill in exercises
        while max_intensity <= self.intensity && generated_exercises <= number_of_exercises && generated_exercises as u32 <= max_exercises {
                        
            let next_exercise = exercise_group.next();
            match next_exercise {
                None => break,
                Some(exercise) => {
                    let new_exercise = exercise.1;
                    generated_exercises += 1;
                    max_intensity += new_exercise.intensity.clone();
                    full_duration += new_exercise.duration.clone();
                    self.excercises.insert(new_exercise.exercise.clone(), new_exercise); // add exercises to the exercises hashmap
                }
            }
        }

        let average_exercise_intensity = self.intensity as f32 / generated_exercises as f32;
        let mut max_intensity = 0;

        // generate the reps for every single exercise
        // we calculate the average intensity for every exercise and 
        // divide this by the intensity of a single exercise to get the amount of exercises we need to do
        for exercise in self.excercises.values_mut() {
            let reps = (average_exercise_intensity / exercise.intensity as f32).round() as u32;
            exercise.set_repetitions(reps); // also calculates the intensity and duration again
            max_intensity += exercise.intensity;
            full_duration += exercise.duration;
        }

        self.intensity = max_intensity;
        self.duration = full_duration;
    }

    pub fn new_settings(&mut self, path: &str) {
        let mut settings_file = File::open(path).unwrap();
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents).unwrap(); 

        self.settings = HashMap::new();
        for line in contents.lines() {
            if !line.starts_with("#") { // exclude comments     
                let line_items: Vec<&str> = line.split(":").collect();
                let key = line_items[0].trim().to_string();
                let value = line_items[1].trim().to_string();
                self.settings.insert(key, value);
            }
        }
    }

    pub fn display_workout(&self) {
        println!("Your Workout");
        println!("-------------------------");
        print!("Duration: ca. {}", self.duration/60);
        print!(" | ");
        println!("Intensity: {}", self.intensity);
        println!("");
        for exercise in self.excercises.values() {
            println!("{}: {}x", exercise.exercise, exercise.repetitions());
        }
    }

    fn generate_breaks() {
        // make breaks
    }

    pub fn save(&self) {
        let connection = sqlite::open("exercises.db").unwrap();
        let time = SystemTime::now(); // need good conversion for system time into sqlite datatype
        let mut cursor = connection.prepare("INSERT VALUE INTO workouts (date, intensity, duration, section, estimated_duration) VALUES (:date, :intensity, :duration, :section, :estimated);").unwrap().into_cursor();
        cursor.bind_by_name(vec![(":date", Value::String(String::from("test"))), 
                                 (":intensity", Value::Integer(self.intensity as i64)), 
                                 (":duration", Value::Integer(self.duration as i64)),
                                 (":section", Value::String(self.section.to_string())),
                                 (":estimated", Value::Integer(self.duration as i64)) ]).unwrap();
        cursor.next().unwrap();
    }
}