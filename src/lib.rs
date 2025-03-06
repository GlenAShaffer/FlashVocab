pub mod flasher{
    use std::time::{Duration, SystemTime, SystemTimeError};
    //Fact struct
    //This struct represents a prompt and an expected answer, as well as how many times the user has answered it correctly in a row, and the last time the fact was checked.
    pub struct Fact{
        prompt: String,
        answer: String,
        score: u32,
        last_checked: SystemTime,
    }

    impl Fact {
        pub fn check(&mut self, r: String) -> bool {
            self.last_checked = SystemTime::now();
            if r == self.answer {
                self.score += 1;
                return true;
            } else {
                self.score = 0;
                return false;
            }
        }

        pub fn get_score(&self) -> u32{
            self.score
        }

        //This function returns the duration since this fact was last checked.
        pub fn get_duration(&self) -> Result<Duration, SystemTimeError>{
            self.last_checked.elapsed()
        }
    }

    //This functions as a constructor for facts
    pub fn new_fact(prompt: &str, answer: &str, score: u32) -> Fact {
        let f = Fact {
            prompt: String::from(prompt),
            answer: String::from(answer),
            score,
            last_checked: SystemTime::now(),
        };

        f
    }

    //Lesson struct
    pub struct Lesson {
        name: String,
        facts: Vec<Fact>,
    }

    impl Lesson {
        pub fn get_name(&self) -> String{
            String::from(&self.name)
        }
    }

    pub fn new_lesson(name: &str, facts: Vec<Fact>) -> Lesson {
        let l = Lesson {
            name: String::from(name),
            facts,
        };

        l
    }
}

//Unit tests
#[cfg(test)]
mod tests{
    use crate::flasher::*;
    use std::time::{Duration, SystemTime};
    use std::thread::sleep;

    //Fact struct tests
    #[test]
    fn fact_correct_answer_returns_true(){
        let mut f = new_fact("Test prompt", "Test answer", 0);

        let test_answer = String::from("Test answer");

        assert!(f.check(test_answer));
    }

    #[test]
    fn fact_wrong_answer_returns_false(){
        let mut f = new_fact("Test prompt", "Test answer", 0);

        let test_answer = String::from("Not test answer");

        assert!(!f.check(test_answer));
    }

    #[test]
    fn fact_get_score_returns_score(){
        let f = new_fact("Test prompt", "Test answer", 0);

        assert_eq!(f.get_score(), 0);
    }

    #[test]
    fn fact_correct_check_increments_score(){
        let mut f = new_fact("Test prompt", "Test answer", 0);

        f.check(String::from("Test answer"));

        assert_eq!(f.get_score(), 1);
    }

    #[test]
    fn fact_incorrect_check_resets_score(){
        let mut f = new_fact("Test prompt", "Test answer", 10);

        f.check(String::from("Not the test answer"));

        assert_eq!(f.get_score(), 0);
    }

    #[test]
    fn fact_get_duration_returns_duration(){
        let f = new_fact("Test prompt", "Test answer", 0);
        let time_result = f.get_duration();
        
        match time_result {
            Ok(duration) => assert!(!duration.is_zero()),
            Err(error) => panic!("Problem calculating duration: {error:?}"),
        };
    }

    #[test]
    fn fact_check_updates_duration(){
        let mut f = new_fact("Test prompt", "Test answer", 0);
        let reference_time = SystemTime::now();

        sleep(Duration::from_millis(500));
        f.check(String::from("Test Answer"));

        let fact_duration: Duration;
        let fact_time_result = f.get_duration();
        let reference_time_result = reference_time.elapsed();

        match fact_time_result {
            Ok(duration) => fact_duration = duration,
            Err(error) => panic!("Problem calculating duration: {error:?}"),
        }

        match reference_time_result {
            Ok(duration) => assert!(duration.checked_sub(fact_duration) > Some(Duration::from_millis(250))),
            Err(error) => panic!("Problem calculating duration: {error:?}"),
        }
    }

    #[test]
    //Lesson struct tests
    fn lesson_get_name_returns_name(){
        let f0 = new_fact("Test prompt0", "Test answer0", 0);
        let f1 = new_fact("Test prompt1", "Test answer1", 0);

        let mut lessons = Vec::new();
        lessons.push(f0);
        lessons.push(f1);

        let l = new_lesson("Test Lesson", lessons);
        assert_eq!(l.get_name(), "Test Lesson");
    }
}