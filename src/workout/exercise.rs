use crate::workout::body_section as body_section;
use std::collections::HashMap;
use sqlite::{ self, Value };
use serde::{ Serialize, Deserialize };

// Excerices defintion
#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub exercise: String,
    pub duration: u32,
    repetitions: u32,
    pub intensity: u32,
    pub section: body_section::BodySection,
}

impl Exercise {
    pub fn new(name: String ,intensity: u32, duration: u32, section: body_section::BodySection) -> Exercise {
        Exercise { exercise: name, duration , repetitions: 0, intensity, section }
    }

    pub fn set_repetitions(&mut self, reps: u32) {
        self.repetitions = reps;
        self.duration *= self.repetitions;
        self.intensity *= self.repetitions;
    }

    pub fn get_group(section: body_section::BodySection) -> HashMap<String, Exercise> {
        let mut group: HashMap<String, Exercise> = HashMap::new();

        let connection = sqlite::open("/home/cesar/Projekte/Rust/workme/src/exercises.db").unwrap();
        let mut cursor = connection.prepare("SELECT * FROM exercises WHERE section = ?;").unwrap().into_cursor();
        cursor.bind(&[Value::String(section.to_string().to_lowercase())]).unwrap();

        while let Some(row) = cursor.next().unwrap() {
            group.insert(
                String::from(row[0].as_string().unwrap()), 
                Exercise::new(String::from(
                    row[0].as_string().unwrap()),
                    row[2].as_integer().unwrap() as u32, 
                    row[3].as_integer().unwrap() as u32, 
                    section.clone()
                )
            );
        }

        return group
    }

    pub fn repetitions(&self) -> u32 {
        self.repetitions
    }
}