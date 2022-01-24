extern crate chrono;

pub mod body_section;
mod exercise;


use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use rand::Rng;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use sqlite::{ self, Value };
use serde::{ Deserialize, Serialize };


// Workout definition

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
    pub intensity: u32,
    pub section: body_section::BodySection,
    pub duration: u32,
    pub excercises: HashMap<String, exercise::Exercise>,
    pub breaks: f32,
    completed: bool,
    settings: HashMap<String, String>,
    statistics: HashMap<String, String>
}

impl Workout {
    pub fn new(intensity: u32, section: body_section::BodySection) -> Workout {
        let connection = sqlite::open("/home/cesar/Projekte/Rust/workme/src/user.db").unwrap();

        let statement = connection.prepare("SELECT * FROM workouts WHERE completed = 0").unwrap();
        let count = statement.column_count();

        let mut cursor = statement.into_cursor();
        let json_workout = cursor.next().unwrap().unwrap()[2].as_string().unwrap();

        if count > 0 {
            println!("{}", json_workout);
            let workout: Workout = serde_json::from_str(&json_workout).unwrap();
            println!{"{}", workout.intensity};
            Workout { 
                intensity: workout.intensity, 
                section: workout.section,
                duration: workout.duration, 
                excercises: workout.excercises, 
                breaks: workout.breaks, 
                settings: workout.settings,
                statistics: workout.statistics,
                completed: workout.completed,
            }

        } else {
            let mut settings = HashMap::new();
            settings.insert(String::from("maximum exercises"), String::from("10"));
            settings.insert(String::from("maximum repetitions"), String::from("50"));

            let mut statistics = HashMap::new();
            statistics.insert(String::from("complete intensity"), String::from("0"));

            Workout { 
                intensity, 
                section, 
                duration: 0, 
                excercises: HashMap::new(), 
                breaks: 4.0, settings, 
                statistics, 
                completed: false 
            }
        }
    }

    pub fn current() {
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
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into(); // convert time to string
        let date = datetime.format("%d/%m/%Y %T").to_string();
        let workout_json = serde_json::to_string(&self).unwrap();

        let connection = sqlite::open("/home/cesar/Projekte/Rust/workme/src/user.db").unwrap();
        let mut cursor = connection.prepare("INSERT INTO workouts (data, date, completed) VALUES (:data, :date, :completed);").unwrap().into_cursor();
        cursor.bind_by_name(vec![(":data", Value::String(workout_json)), (":date", Value::String(date)), (":completed", Value::Integer(self.completed as i64))]).unwrap();
        cursor.next().unwrap();
    }

    pub fn completed(&mut self, status: bool) {
        self.completed = status;
    }
}