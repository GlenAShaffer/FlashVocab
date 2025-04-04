pub mod flasher{
    use std::time::{Duration, SystemTime, SystemTimeError};

    //Fact struct
    //This struct represents a prompt and an expected answer, as well as how many times the user has answered it correctly in a row, and the last time the fact was checked.
    #[derive(Debug)]
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

    impl PartialEq<Fact> for &mut Fact {
        fn eq(&self, other: &Fact) -> bool {
            return self.prompt == other.prompt && self.answer == other.answer;
        }
    }

    impl Clone for Fact {
        fn clone(&self) -> Self {
            Fact {
                prompt: String::from(&self.prompt),
                answer: String::from(&self.answer),
                score: self.score,
                last_checked: self.last_checked,
            }
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
    #[derive(Debug)]
    pub struct Lesson {
        name: String,
        facts: Vec<Fact>,
        goal: usize,
    }

    impl Lesson {
        pub fn get_name(&self) -> String{
            String::from(&self.name)
        }

        pub fn get_facts(&self) -> Vec<Fact> {
            self.facts.clone()
        }

        pub fn get_fact(&mut self, index: usize) -> &mut Fact{
            self.facts.get_mut(index).expect(&format!("Cannot get fact {} from lesson.", index))
        }

        //This function selects the next fact in the lesson to review based on which fact has the lowest score, and breaks ties with duration since last review.
        pub fn next_fact(&mut self) -> &mut Fact{
            //Assign the index of the next fact to zero
            let mut next_index = 0;
            //Assign the initial minimum score to the score of the first fact in the lesson
            let mut min_score: u32 = self.facts.get(next_index).expect(&format!("Cannot get fact {} from lesson.", next_index)).get_score();
            
            //Iterate through each fact in the lesson to find the index of the fact with the lowest score.
            for i in 0..self.facts.len() {
                let f = self.facts.get(i).expect(&format!("Cannot get fact {} from lesson.", i));
                let score = f.get_score();
                if score < min_score && i != next_index {
                    min_score = score;
                    next_index = i;
                } else if score == min_score && i != next_index {
                    let lead = self.facts.get(next_index).expect(&format!("Cannot get fact {} from lesson.", next_index));
                    let f_duration = match f.get_duration(){
                        Ok(duration) => duration,
                        Err(error) => panic!("Problem calculating duration: {error:?}"),
                    };
                    let lead_duration = match lead.get_duration(){
                        Ok(duration) => duration,
                        Err(error) => panic!("Problem calculating duration: {error:?}"),
                    };

                    //Compare durations, the fact with the larger duration should be returned.
                    if f_duration > lead_duration {
                        min_score = score;
                        next_index = i;
                    }
                }
            }

            //Return a mutable pointer to the next fact using the index that was found.
            self.facts.get_mut(next_index).expect(&format!("Cannot get fact {} from lesson.", next_index))
        }

        //This functions returns the degree of completion of the lesson as a floating point number between 0 and 1.
        pub fn completion(&mut self) -> f32 {
            let total = (self.get_facts().len() * self.goal) as f32;
            let mut sum: u32 = 0;

            for i in 0..self.get_facts().len(){
                sum += self.get_fact(i).get_score();
            }

            let f_sum = sum as f32;

            f_sum / total * 1.0_f32
        }
    }

    impl PartialEq<Lesson> for &mut Lesson {
        fn eq(&self, other: &Lesson) -> bool {
            return self.name == other.name;
        }
    }

    impl Clone for Lesson {
        fn clone(&self) -> Self {
            Lesson {
                name: String::from(&self.name),
                facts: self.facts.clone(),
                goal: self.goal,
            }
        }
    }

    pub fn new_lesson(name: &str, facts: Vec<Fact>) -> Lesson {
        let l = Lesson {
            name: String::from(name),
            facts,
            goal: 3,
        };

        l
    }

    pub fn new_lesson_goal(name: &str, facts: Vec<Fact>, goal: usize) -> Lesson {
        let l = Lesson {
            name: String::from(name),
            facts,
            goal,
        };

        l
    }

    pub struct Course {
        name: String,
        lessons: Vec<Lesson>,
    }

    impl Course {
        pub fn get_lesson(&mut self, index: usize) -> &mut Lesson{
            self.lessons.get_mut(index).expect(&format!("Cannot get lesson {} from course.", index))
        }
    }

    pub fn new_course(name: &str, lessons: Vec<Lesson>) -> Course{
        let c = Course {
            name: String::from(name),
            lessons,
        };

        c
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

    #[test]
    fn lesson_next_fact_returns_lowest_score(){
        let f0 = new_fact("Test prompt0", "Test answer0", 2);
        let f1 = new_fact("Test prompt1", "Test answer1", 0);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());

        let mut l = new_lesson("Test Lesson", facts);
        assert_eq!(l.next_fact(), f1);
    }

    #[test]
    fn lesson_next_fact_tie_returns_oldest(){
        let f0 = new_fact("Test prompt0", "Test answer0", 0);
        let f1 = new_fact("Test prompt1", "Test answer1", 1);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());

        let mut l = new_lesson("Test Lesson", facts);
        
        sleep(Duration::from_millis(500));

        l.next_fact().check(String::from("Test answer0"));

        assert_eq!(l.next_fact(), f1);
    }

    #[test]
    fn facts_len_returns_len(){
        let f0 = new_fact("Test prompt0", "Test answer0", 0);
        let f1 = new_fact("Test prompt1", "Test answer1", 1);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());

        let l = new_lesson("Test Lesson", facts);

        assert_eq!(l.get_facts().len(), 2);
    }

    #[test]
    fn lesson_completion_returns_complete() {
        let f0 = new_fact("Test prompt0", "Test answer0", 3);
        let f1 = new_fact("Test prompt1", "Test answer1", 3);
        let f2 = new_fact("Test prompt2", "Test answer2", 3);
        let f3 = new_fact("Test prompt3", "Test answer3", 3);
        let f4 = new_fact("Test prompt4", "Test answer4", 3);
        let f5 = new_fact("Test prompt5", "Test answer5", 3);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());
        facts.push(f2.clone());
        facts.push(f3.clone());
        facts.push(f4.clone());
        facts.push(f5.clone());

        let mut l = new_lesson("Test Lesson", facts);

        assert_eq!(l.completion().floor(), 1.0_f32.floor());
    }

    #[test]
    fn lesson_completion_returns_empty() {
        let f0 = new_fact("Test prompt0", "Test answer0", 0);
        let f1 = new_fact("Test prompt1", "Test answer1", 0);
        let f2 = new_fact("Test prompt2", "Test answer2", 0);
        let f3 = new_fact("Test prompt3", "Test answer3", 0);
        let f4 = new_fact("Test prompt4", "Test answer4", 0);
        let f5 = new_fact("Test prompt5", "Test answer5", 0);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());
        facts.push(f2.clone());
        facts.push(f3.clone());
        facts.push(f4.clone());
        facts.push(f5.clone());

        let mut l = new_lesson("Test Lesson", facts);

        assert_eq!(l.completion().round(), 0.0_f32.round());
    }

    #[test]
    fn course_get_lesson_returns_lesson() {
        let f0 = new_fact("Test prompt0", "Test answer0", 0);
        let f1 = new_fact("Test prompt1", "Test answer1", 1);

        let mut facts = Vec::new();
        facts.push(f0.clone());
        facts.push(f1.clone());

        let mut l0 = new_lesson("Test Lesson 0", facts.clone());
        let mut l1 = new_lesson("Test Lesson 1", facts.clone());
        let mut l2 = new_lesson("Test Lesson 2", facts);

        let mut lessons = Vec::new();
        lessons.push(l0.clone());
        lessons.push(l1.clone());
        lessons.push(l2.clone());

        let mut c = new_course("Test Course 0", lessons);

        assert_eq!(c.get_lesson(0), l0);
    }
}
