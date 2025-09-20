use crate::student::Student;

#[derive(Debug, Clone)]
pub struct Classroom {
    students: Vec<Student>,
    id: u16,
    teacher_id: u16,
    name: String,
}

impl Classroom {
    pub fn new(id: u16, teacher_id: u16, name: String) -> Self {
        Classroom {
            students: Vec::new(),
            id,
            teacher_id,
            name,
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn student_cont(&self) -> usize {
        self.students.len()
    }

    pub fn suject_average(&self) -> (f64, f64, f64, f64, f64) {
        if self.students.is_empty() {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
        let total_japanese: u32 = self.students.iter().map(|s| s.japanese as u32).sum();
        let total_math: u32 = self.students.iter().map(|s| s.math as u32).sum();
        let total_english: u32 = self.students.iter().map(|s| s.english as u32).sum();
        let total_science: u32 = self.students.iter().map(|s| s.science as u32).sum();
        let total_social: u32 = self.students.iter().map(|s| s.social as u32).sum();

        let count = self.students.len() as u32;

        (
            total_japanese as f64 / count as f64,
            total_math as f64 / count as f64,
            total_english as f64 / count as f64,
            total_science as f64 / count as f64,
            total_social as f64 / count as f64,
        )
    }
    pub fn highest_score(&self) -> Option<&Student> {
        self.students.iter().max_by_key(|s| s.total_score())
    }

    pub fn lowest_score(&self) -> Option<&Student> {
        self.students.iter().min_by_key(|s| s.total_score())
    }

    pub fn above_average_students(&self) -> Vec<&Student> {
        let average = self.average_score();
        self.students
            .iter()
            .filter(|s| s.total_score() as f32 > average)
            .collect()
    }

    pub fn average_score(&self) -> f32 {
        if self.students.is_empty() {
            return 0.0;
        }
        let total_score: f32 = self.students.iter().map(|s| s.total_score() as f32).sum();
        total_score / self.students.len() as f32
    }
}
