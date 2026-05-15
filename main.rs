use core::fmt;

// #[derive(Debug,Default)]
// enum Departments {
//     Engineering,
//     Business,
//     ComputerScience,
//     Archtitecture,
// }
#[derive(Debug)]
struct Person {
    name: String,
    id: u32,
    courses: Vec<String>,
    // department: Departments,
    department: String,
}
enum AccessLevel {
    StudentOnly,
    StaffOnly,
    AdminOnly,
    Public,
}
enum Staffs {
    Student,
    Teacher,
    Staff,
    Principal,
}


#[derive(Debug)]
struct Student {
    person: Person,
    grade: Option<u8>,
    attendance_percentage: f64,
}

struct Teacher {
    person: Person,
    years_experience: u8,
    salary: f64,
}
struct Staff {
    person: Person,
    department: String,
    salary: f64,
}

fn main() {
    // let person_1 = Person::new_person("benjamin".to_owned(), 1);
    let person_1 = Person::default();
    println!("{:?}", person_1);

    println!("Hello, world!");

    let student_1 = Student::student_new(person_1, Some(33), 89.0);
    println!("student 1:{}",student_1);
}

impl Person {
    fn new_person(name: String, id: u32, courses: Vec<String>, department: String) -> Self {
        Self {
            name,
            id,
            courses,
            department,
        }
    }
}

impl Student {
    fn student_new(person: Person, grade: Option<u8>, attendance_percentage: f64) -> Self {
        Self {
            person,
            grade,
            attendance_percentage,
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        // fn default(&mut self) -> Self {
        Person {
            name: "john doe".to_owned(),
            id: 0,
            courses: vec!["Mathematics".to_owned(), "English".to_owned()],
            department: String::from("Engineering"),
            // department: Departments::Business,
        }
    }
}

// impl Default for Departments{
//     fn default() -> Self {
//         Departments::ComputerScience
//     }
// }

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Student info:\nstudent:{}\ngrade:{}\nattendance:{}\n",
            self.person,
            self.grade.unwrap_or_default(),
            self.attendance_percentage
        )
    }
}


impl fmt::Display for Person{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(
            f,"Person info:name:{}\nid:{}\ncourse:{}\ndepartment:{}\n",
            self.name,
            self.id,
            self.courses.join(","),
            self.department
        )
    }
}
