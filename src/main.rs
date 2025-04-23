use std::{collections::HashMap, io};
fn get_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again");
    }

    let input = buffer.trim().to_owned();

    if &input == "" { None } else { Some(input) }
}

fn get_u8() -> Option<u8> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed = input.parse::<u8>();
    match parsed {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u8,
}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self { class: HashMap::new() }
    }

    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }

    fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    fn edit(&mut self, name: &str, age: u8) -> bool {
        if let Some(student) = self.class.get_mut(name) {
            student.age = age;
            true
        } else {
            false
        }
    }
}

mod manager {
    use crate::*;

    pub fn add_student(students: &mut Students) {
        println!("Enter student name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter student age: ");
        let age = match get_u8() {
            Some(number) => number,
            None => return,
        };
        let student = Student { name, age };

        students.add(student)
    }

    pub fn view_all(students: &Students) {
        let students = students.view_all();
        for student in students {
            println!("{:?}", student);
            println!("Name: {}", student.name);
            println!("Age: {}", student.age);
            println!("===========================================");
        }
    }

    pub fn delete(students: &mut Students) {
        println!("Enter student name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let removed = students.remove(&name);
        if removed {
            println!("Student removed");
        } else {
            println!("Student not found");
        }
    }

    pub fn edit(students: &mut Students) {
        println!("Enter student name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter student age: ");
        let age = match get_u8() {
            Some(number) => number,
            None => return,
        };
        let edited = students.edit(&name, age);
        if edited {
            println!("Student edited");
        } else {
            println!("Student not found");
        }
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("============ Student Management ============");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("===========================================");
        println!("Please choose: ");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();

    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data:");

        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view_all(&students),
            Some(Manager::EditStudent) => manager::edit(&mut students),
            Some(Manager::DeleteStudent) => manager::delete(&mut students),
            None => return,
        }
    }
}
