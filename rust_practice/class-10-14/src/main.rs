mod student;
use student::Student;

fn main() {
    let name = "Ryan".to_string();
    let major = "Computer Science".to_string();

    let mut stud = Student::new_student(name, major);
    stud.introduction();

    stud.display_name();
    stud.display_major();

    let new_major = "Bio".to_string();
    stud.change_major(new_major);
}
