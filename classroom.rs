use std::{collections::HashMap};
use crate::student::{Student};

pub struct Classroom {
    students: HashMap<String, Student>,
}

impl Classroom {
    pub fn new() -> Self {
        Classroom {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: String) -> bool {
        if self.students.contains_key(&name) {
            false
        }else{
            let student = Student::new(name.clone());
            self.students.insert(name,student);
            true
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
        for (_ ,student) in &self.students {
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
        }else {
            false
        }
    }

    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    pub fn active_student_count(&self) -> usize {
        let mut count = 0;
        for (_, student) in &self.students {
            if student.is_active() {
                count+=1;
            }
        }
        count
    }
}