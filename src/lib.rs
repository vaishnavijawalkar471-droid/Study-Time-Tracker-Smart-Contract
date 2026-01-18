// Study Time Tracker - Track your study hours
pub struct StudyTracker {
    hours: i32,
    student: String,
}

impl StudyTracker {
    pub fn new(student: String) -> Self {
        StudyTracker { hours: 0, student }
    }

    pub fn log_hour(&mut self, caller: &String) -> Result<i32, String> {
        if caller != &self.student {
            return Err("Not your tracker".to_string());
        }
        self.hours += 1;
        Ok(self.hours)
    }

    pub fn get_hours(&self) -> i32 {
        self.hours
    }

    pub fn reset(&mut self, caller: &String) -> Result<(), String> {
        if caller != &self.student {
            return Err("Not your tracker".to_string());
        }
        self.hours = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = "Sarah".to_string();
        let mut tracker = StudyTracker::new(student.clone());

        assert_eq!(tracker.log_hour(&student).unwrap(), 1);
        assert_eq!(tracker.log_hour(&student).unwrap(), 2);
        assert_eq!(tracker.get_hours(), 2);
        
        tracker.reset(&student).unwrap();
        assert_eq!(tracker.get_hours(), 0);
    }
}
