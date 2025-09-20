#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    id: u16,
    japanese: u16,
    math: u16,
    english: u16,
    science: u16,
    social: u16,
}

impl Student {
    pub fn new(name: String, id: u16, japanese: u16, math: u16, english: u16, sience: u16, social: u16) ->Self {
        Student {
            name,
            id,
            japanese: japanese,
            math,
            english,
            science: sience,
            social,
        }
    }

    fn total_score(&self) -> u16 {
        self.japanese + self.math + self.english + self.science + self.social
    }

    fn average_score(&self) -> f32 {
        self.total_score() as f32 / 5.0
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Japanese: {}", self.japanese);
        println!("Math: {}", self.math);
        println!("English: {}", self.english);
        println!("Science: {}", self.science);
        println!("Social: {}", self.social);
        println!("Total Score: {}", self.total_score());
        println!("Average Score: {}", self.average_score());
    }

}
