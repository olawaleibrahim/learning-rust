struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter().filter(
            |&e| e.student.name == self.name
        ).map(
            |e| e.course.name.clone()
        ).collect()
    }
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course  // student and course should have the same lifetime
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment {student, course}
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self,
              student: &'a Student,
              course: &'a Course
    ) {
        self.enrollments.push(
            Enrollment::new(student, course)
        )
    }
}

pub fn results() {
    let student = Student{name: "Olawale".into()};
    let course = Course { name: "Intro to Rust".into() };
    let course2 = Course { name: "Machine learning with Rust".into() };

    let mut p = Platform::new();
    p.enroll(&student, &course);
    p.enroll(&student, &course2);

    for c in student.courses(p) {
        println!("{} is taking {}", student.name, c);
    }
}