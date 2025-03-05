pub mod flasher{
    use std::time::{Duration, SystemTime, SystemTimeError};
    //Fact enum
    pub struct Fact{
        prompt: String,
        answer: String,
        score: u32,
        last_checked: SystemTime,
    }

    impl Fact{
        pub fn check(&mut self, r: String) -> bool {
            self.last_checked = SystemTime::now();
            r == self.answer
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

        return f;
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
    fn fact_get_duration_returns_duration(){
        let f = new_fact("Test prompt", "Test answer", 0);
        let time_result = f.get_duration();
        
        match time_result {
            Ok(duration) => assert!(!duration.is_zero()),
            Err(error) => panic!("Problem calculating duration: {error:?}"),
        };
    }

    #[test]
    fn check_updates_duration(){
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
}