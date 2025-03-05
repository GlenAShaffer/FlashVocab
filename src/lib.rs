
pub mod flasher{
    //Fact enum
    pub struct Fact{
        prompt: String,
        answer: String,
    }

    impl Fact{
        pub fn check(&self, r: String) -> bool {
            return r == self.answer;
        }
    }

    //This functions as a constructor for facts
    pub fn new_fact(p: &str, a: &str) -> Fact {
        let f = Fact {
            prompt: String::from(p),
            answer: String::from(a),
        };

        return f;
    }
}

//Unit tests
#[cfg(test)]
mod tests{
    use super::*;

    //Fact struct tests
    #[test]
    fn fact_correct_answer_returns_true(){
        let f = flasher::new_fact("Test prompt", "Test answer");

        let test_answer = String::from("Test answer");

        assert_eq!(f.check(test_answer), true);
    }

    #[test]
    fn fact_wrong_answer_returns_false(){
        let f = flasher::new_fact("Test prompt", "Test answer");

        let test_answer = String::from("Not test answer");

        assert_eq!(f.check(test_answer), false);
    }
}