#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub id: u8,
    pub japnese: u8,
    pub math: u8,
    pub english: u8,
    pub sience: u8,
    pub social: u8,
}

impl Student {
    pub fn new(name: String, id: u8, japnese: u8, math: u8, english: u8, sience: u8, social: u8) ->Self {
        Student {
            name,
            id,
            japnese,
            math,
            english,
            sience,
            social,
        }
    }

    fn total_score(&self) -> u8 {
        self.japnese + self.math + self.english + self.sience + self.social
    }

    fn average_score(&self) -> f32 {
        self.total_score() as f32 / 5.0
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Japanese: {}", self.japnese);
        println!("Math: {}", self.math);
        println!("English: {}", self.english);
        println!("Science: {}", self.sience);
        println!("Social: {}", self.social);
        println!("Total Score: {}", self.total_score());
        println!("Average Score: {}", self.average_score());
    }

}
