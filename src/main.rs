mod classroom;
mod student;
use crate::classroom::Classroom;
use crate::student::Student;

fn main() {
    let mut class1 = Classroom::new(1, 10, "Otaka".to_string());

    class1.add_student(Student::new("Tanaka".to_string(), 1, 13, 20, 100, 43));
    class1.add_student(Student::new("Sato".to_string(), 2, 24, 56, 75, 55));
    class1.add_student(Student::new("Suzuki".to_string(), 3, 3, 34, 34, 1));
    class1.add_student(Student::new("Yamada".to_string(), 4, 43, 53, 1, 56));
    class1.add_student(Student::new("Abe".to_string(), 5, 24, 56, 75, 55));
    class1.add_student(Student::new("Sakita".to_string(), 6, 3, 34, 34, 1));

    class1
        .above_average_students()
        .iter()
        .for_each(|s| s.display());
    print!("student count: {}\n", class1.average_score());
    //  print!("student count: {},{},{},{},{}",class1.suject_average());
    class1.highest_score().unwrap().display();
    class1.lowest_score().unwrap().display();
    print!("student count: {}\n", class1.student_cont());

    // ここから編集しようとしました
}
