use crate::student::{Status, Student};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Classroom {
    students: HashMap<String, Student>,
}

impl Classroom {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: String) -> bool {
        match self.students.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(Student::new(name));
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    pub fn find_student(&self, name: &str) -> Option<&Student> {
        self.students.get(name)
    }

    pub fn find_student_mut(&mut self, name: &str) -> Option<&mut Student> {
        self.students.get_mut(name)
    }

    pub fn remove_student(&mut self, name: &str) -> Option<Student> {
        self.students.remove(name)
    }

    pub fn print_all(&self) {
        for student in self.students.values() {
            student.print();
        }
    }

    pub fn rename_student(&mut self, old_name: &str, new_name: String) -> bool {
        if self.students.contains_key(&new_name) {
            return false;
        }
        if let Some(mut student) = self.remove_student(old_name) {
            student.rename(new_name.clone());
            self.students.insert(new_name, student);
            true
        } else {
            false
        }
    }

    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    pub fn active_student_count(&self) -> usize {
        let mut count = 0;
        for student in self.students.values() {
            if student.is_active() {
                count += 1;
            }
        }
        count
    }

    pub fn execute(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        match parts[0] {
            "add" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    self.add_student(name.to_string());
                    return;
                }
            }
            "score" => {
                if parts.len() == 3 {
                    let name = parts[1];
                    if let Ok(score) = parts[2].parse::<i32>() {
                        if let Some(student) = self.find_student_mut(name) {
                            student.add_score(score);
                            return;
                        }
                    }
                }
            }
            "status" => {
                if parts.len() == 3 {
                    let name = parts[1];
                    let status = parts[2];
                    if let Some(student) = self.find_student_mut(name) {
                        match status {
                            "active" => {
                                student.change_status(Status::Active);
                                return;
                            }
                            "leave" => {
                                student.change_status(Status::Leave);
                                return;
                            }
                            "graduated" => {
                                student.change_status(Status::Graduated);
                                return;
                            }
                            _ => {}
                        }
                    }
                }
            }
            "show" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    if let Some(student) = self.find_student(name) {
                        student.print();
                        return;
                    }
                }
            }
            "remove" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    self.remove_student(name);
                    return;
                }
            }
            "list" => {
                println!("list");
                if parts.len() == 1 {
                    self.print_all();
                    return;
                }
            }
            _ => {}
        }

        println!("알 수 없는 명령어");
    }
}
