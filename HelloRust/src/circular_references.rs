struct Student<'a> {
    name: String,
    courses: Vec<&'a Course<'a>>
}

impl<'a> Student<'a> {
    fn new(name: &str) -> Student<'a> {
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course<'a> {
    name: String,
    students: Vec<&'a Student<'a>>
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(&'a mut self,
        student: &'a mut Student<'a>) {
        student.courses.push(self);
        self.students.push(student);
        // RefCell
    }
    
}

pub fn results() {
    let john = Student::new("John");
    let course = Course::new("Rust Course");

    course.add_student(john)
}