#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub number: u16,
    pub japanese: u16,
    pub math: u16,
    pub english: u16,
    pub science: u16,
}

impl Student {
    pub fn new(name: String, id: u16, japanese: u16, math: u16, english: u16, sience: u16) ->Self {
        Student {
            name,
            number: id,
            japanese: japanese,
            math,
            english,
            science: sience,
        }
    }

    pub fn total_score(&self) -> u16 {
        self.japanese + self.math + self.english + self.science 
    }

    pub fn average_score(&self) -> f32 {
        self.total_score() as f32 / 4.0
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("ID: {}", self.number);
        println!("Japanese: {}", self.japanese);
        println!("Math: {}", self.math);
        println!("English: {}", self.english);
        println!("Science: {}", self.science);
        println!("Total Score: {}", self.total_score());
        println!("Average Score: {}", self.average_score());
    }

}
