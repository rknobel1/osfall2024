pub struct Student {
    name: String, 
    major: String,
}

impl Student {
    pub fn new_student(name: String, major: String) -> Student {
        Student {
            name: name,
            major: major,
        }   
    }

    pub fn introduction(&self) {
        println!("Hi! I am {} and I am majoring in {}", self.name, self.major)
    }

    pub fn display_name(&self) {
        println!("Students name is: {}", self.name);
    }

    pub fn display_major(&self) {
        println!("Students major is: {}", self.major);
    }

    pub fn change_major(&mut self, m: String) {
        self.major = m;
        println!("Students major is changed to: {}", self.major);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let stud = Student::new_student("Ryan".to_string(), "Computer Science".to_string());
        assert_eq!(stud.name, "Ryan".to_string());
        assert_eq!(stud.major, "Computer Science".to_string());
    }

    #[test]
    fn test_change_major() {
        let mut stud = Student::new_student("Ryan".to_string(), "Computer Science".to_string());
        stud.change_major("Bio".to_string());
        assert_eq!(stud.major, "Bio".to_string());
    }
}