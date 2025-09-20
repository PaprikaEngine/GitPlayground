mod student;
use crate::student::Student;

fn main() {
    let tanaka = Student::new(
        "Tanaka".to_string(),
        1,
        13,
        20,
        100,
        43,
    );
    tanaka.display();
}
