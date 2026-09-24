use crate::student::{Status, Student};
use crate::summary::Summary;
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

    pub fn higher_average<'a>(first: &'a Student, second: &'a Student) -> &'a Student {
        if first.average() > second.average() {
            return first;
        } else {
            second
        }
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
        self.students.values().filter(|student| student.is_active()).count()
    }

    pub fn execute(&mut self, command: &str) -> Result<(), CommandError> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CommandError::WrongArgsNum);
        }
        match parts[0] {
            "add" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    if !self.add_student(name.to_string()) {
                        return Err(CommandError::DuplicatedStudent);
                    }
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            "score" => {
                if parts.len() == 3 {
                    let name = parts[1];
                    let score = parts[2].parse::<i32>()?;
                    let student = self
                        .find_student_mut(name)
                        .ok_or(CommandError::NotFindStudent)?;
                    if !student.add_score(score) {
                        return Err(CommandError::CannotAddScore);
                    }
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            "status" => {
                if parts.len() == 3 {
                    let name = parts[1];
                    let status = parts[2];
                    let student = self
                        .find_student_mut(name)
                        .ok_or(CommandError::NotFindStudent)?;

                    match status {
                        "active" => student.change_status(Status::Active),
                        "leave" => student.change_status(Status::Leave),
                        "graduated" => student.change_status(Status::Graduated),
                        _ => {
                            return Err(CommandError::WrongStatus);
                        }
                    }
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            "show" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    let student = self
                        .find_student(name)
                        .ok_or(CommandError::NotFindStudent)?;
                    student.print();
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            "remove" => {
                if parts.len() == 2 {
                    let name = parts[1];
                    self.remove_student(name)
                        .ok_or(CommandError::NotFindStudent)?;
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            "list" => {
                if parts.len() == 1 {
                    self.print_all();
                } else {
                    return Err(CommandError::WrongArgsNum);
                }
            }
            _ => {
                return Err(CommandError::NonExistentCommand);
            }
        }

        Ok(())
    }
}

impl Summary for Classroom {
    fn summary(&self) -> String {
        format!(
            "학생 수: {} | 재학생 수: {}",
            self.student_count(),
            self.active_student_count()
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    WrongArgsNum,
    NonExistentCommand,
    NotFindStudent,
    DuplicatedStudent,
    FailParse,
    WrongStatus,
    CannotAddScore,
}

impl From<std::num::ParseIntError> for CommandError {
    fn from(_: std::num::ParseIntError) -> Self {
        CommandError::FailParse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classroom_with_alice() -> Classroom {
        let mut classroom = Classroom::new();
        assert!(classroom.add_student("Alice".into()));

        let alice = classroom.find_student_mut("Alice").unwrap();
        assert!(alice.add_score(80));
        alice.change_status(Status::Leave);

        classroom
    }

    fn assert_alice_unchanged(classroom: &Classroom) {
        let alice = classroom.find_student("Alice").unwrap();

        assert_eq!(alice.name(), "Alice");
        assert_eq!(alice.scores(), &[80]);
        assert_eq!(alice.status(), Status::Leave);
    }

    #[test]
    fn new_classroom_is_empty() {
        let classroom = Classroom::new();

        assert_eq!(classroom.student_count(), 0);
        assert_eq!(classroom.active_student_count(), 0);
        assert!(classroom.find_student("Nobody").is_none());
    }

    #[test]
    fn add_student_makes_student_available() {
        let mut classroom = Classroom::new();

        assert!(classroom.add_student("Alice".into()));

        let alice = classroom.find_student("Alice").unwrap();
        assert_eq!(alice.name(), "Alice");
        assert_eq!(alice.status(), Status::Active);
        assert_eq!(classroom.student_count(), 1);
        assert_eq!(classroom.active_student_count(), 1);
    }

    #[test]
    fn duplicate_add_preserves_existing_student() {
        let mut classroom = classroom_with_alice();

        assert!(!classroom.add_student("Alice".into()));

        assert_eq!(classroom.student_count(), 1);
        assert_alice_unchanged(&classroom);
    }

    #[test]
    fn missing_student_operations_preserve_existing_students() {
        let mut classroom = classroom_with_alice();

        assert!(classroom.find_student("Nobody").is_none());
        assert!(classroom.find_student_mut("Nobody").is_none());
        assert!(classroom.remove_student("Nobody").is_none());

        assert_eq!(classroom.student_count(), 1);
        assert_alice_unchanged(&classroom);
    }

    #[test]
    fn remove_student_returns_original_student() {
        let mut classroom = classroom_with_alice();

        let removed = classroom.remove_student("Alice").unwrap();

        assert_eq!(removed.name(), "Alice");
        assert_eq!(removed.scores(), &[80]);
        assert_eq!(removed.status(), Status::Leave);
        assert!(classroom.find_student("Alice").is_none());
        assert_eq!(classroom.student_count(), 0);
    }

    #[test]
    fn rename_updates_lookup_key_and_preserves_student_data() {
        let mut classroom = classroom_with_alice();

        assert!(classroom.rename_student("Alice", "Carol".into()));

        assert!(classroom.find_student("Alice").is_none());

        let carol = classroom.find_student("Carol").unwrap();
        assert_eq!(carol.name(), "Carol");
        assert_eq!(carol.scores(), &[80]);
        assert_eq!(carol.status(), Status::Leave);
        assert_eq!(classroom.student_count(), 1);
    }

    #[test]
    fn rename_to_existing_name_preserves_both_students() {
        let mut classroom = classroom_with_alice();
        assert!(classroom.add_student("Bob".into()));
        assert!(
            classroom
                .find_student_mut("Bob")
                .unwrap()
                .add_score(95)
        );

        assert!(!classroom.rename_student("Alice", "Bob".into()));

        assert_eq!(classroom.student_count(), 2);
        assert_alice_unchanged(&classroom);

        let bob = classroom.find_student("Bob").unwrap();
        assert_eq!(bob.name(), "Bob");
        assert_eq!(bob.scores(), &[95]);
        assert_eq!(bob.status(), Status::Active);
    }

    #[test]
    fn rename_missing_student_preserves_classroom() {
        let mut classroom = classroom_with_alice();

        assert!(!classroom.rename_student("Nobody", "Carol".into()));

        assert!(classroom.find_student("Carol").is_none());
        assert_eq!(classroom.student_count(), 1);
        assert_alice_unchanged(&classroom);
    }

    #[test]
    fn rename_to_same_name_is_rejected_without_changes() {
        let mut classroom = classroom_with_alice();

        assert!(!classroom.rename_student("Alice", "Alice".into()));

        assert_eq!(classroom.student_count(), 1);
        assert_alice_unchanged(&classroom);
    }

    #[test]
    fn active_count_excludes_leave_and_graduated_students() {
        let mut classroom = Classroom::new();

        for (name, status) in [
            ("Alice", Status::Active),
            ("Bob", Status::Leave),
            ("Carol", Status::Graduated),
        ] {
            assert!(classroom.add_student(name.into()));
            classroom
                .find_student_mut(name)
                .unwrap()
                .change_status(status);
        }

        assert_eq!(classroom.student_count(), 3);
        assert_eq!(classroom.active_student_count(), 1);
    }

    #[test]
    fn student_names_are_case_sensitive() {
        let mut classroom = Classroom::new();

        assert!(classroom.add_student("Alice".into()));
        assert!(classroom.add_student("alice".into()));

        assert_eq!(classroom.student_count(), 2);
        assert_eq!(classroom.find_student("Alice").unwrap().name(), "Alice");
        assert_eq!(classroom.find_student("alice").unwrap().name(), "alice");
    }
}
