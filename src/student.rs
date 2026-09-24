use core::fmt;

use crate::summary::Summary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Leave,
    Graduated,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "재학"),
            Status::Leave => write!(f, "휴학"),
            Status::Graduated => write!(f, "졸업"),
        }
    }
}

pub struct Student {
    name: String,
    scores: Vec<i32>,
    status: Status,
}

impl Student {
    pub fn new(name: String) -> Self {
        Self {
            name,
            scores: Vec::new(),
            status: Status::Active,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    pub fn add_score(&mut self, score: i32) -> bool {
        match self.status {
            Status::Active => {
                self.scores.push(score);
                true
            }
            _ => false,
        }
    }
    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn average(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        let sum: i128 = self.scores.iter().map(|&score| i128::from(score)).sum();
        sum as f64 / self.scores.len() as f64
    }

    pub fn highest_score(&self) -> Option<i32> {
        self.scores.iter().max().copied()
    }

    pub fn print(&self) {
        println!("이름: {}", self.name);
        self.print_status();
        if self.scores.is_empty() {
            println!("점수 없음")
        } else {
            print!("점수: ");
            for score in &self.scores {
                print!("{} ", score);
            }
            println!();
        }

        println!("평균: {:.2}", self.average());

        match self.highest_score() {
            Some(score) => println!("최고점: {}", score),
            None => println!("최고점: 없음"),
        }
        println!();
    }

    pub fn remove_last_score(&mut self) -> Option<i32> {
        self.scores.pop()
    }

    pub fn change_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn print_status(&self) {
        println!("상태: {}", self.status.to_string());
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scores(&self) -> &[i32] {
        &self.scores
    }

    pub fn status(&self) -> Status {
        self.status
    }
}

impl Summary for Student {
    fn summary(&self) -> String {
        format!(
            "{} | 평균: {} | 최고점: {} | 상태: {}",
            self.name,
            self.average(),
            self.highest_score().unwrap_or(0),
            self.status.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_student_has_expected_initial_state() {
        let student = Student::new("Alice".into());

        assert_eq!(student.name(), "Alice");
        assert_eq!(student.status(), Status::Active);
        assert!(student.is_active());
        assert!(student.scores().is_empty());
        assert_eq!(student.average(), 0.0);
        assert_eq!(student.highest_score(), None);
    }

    #[test]
    fn active_student_can_add_scores() {
        let mut student = Student::new("Alice".into());

        assert!(student.add_score(80));
        assert!(student.add_score(91));

        assert_eq!(student.scores(), &[80, 91]);
        assert!((student.average() - 85.5).abs() < 1e-9);
        assert_eq!(student.highest_score(), Some(91));
    }

    #[test]
    fn inactive_student_cannot_add_scores() {
        for status in [Status::Leave, Status::Graduated] {
            let mut student = Student::new("Alice".into());
            assert!(student.add_score(80));
            student.change_status(status);

            let added = student.add_score(100);

            assert!(!added, "status: {status:?}");
            assert_eq!(student.scores(), &[80]);
            assert_eq!(student.status(), status);
            assert!(!student.is_active());
        }
    }

    #[test]
    fn reactivated_student_can_add_scores() {
        let mut student = Student::new("Alice".into());
        student.change_status(Status::Leave);
        student.change_status(Status::Active);

        assert!(student.add_score(90));
        assert_eq!(student.scores(), &[90]);
        assert!(student.is_active());
    }

    #[test]
    fn remove_last_score_returns_scores_in_reverse_order() {
        let mut student = Student::new("Alice".into());
        assert!(student.add_score(80));
        assert!(student.add_score(91));

        assert_eq!(student.remove_last_score(), Some(91));
        assert_eq!(student.scores(), &[80]);
        assert_eq!(student.highest_score(), Some(80));
        assert_eq!(student.average(), 80.0);

        assert_eq!(student.remove_last_score(), Some(80));
        assert!(student.scores().is_empty());
        assert_eq!(student.highest_score(), None);
        assert_eq!(student.average(), 0.0);

        assert_eq!(student.remove_last_score(), None);
    }

    #[test]
    fn inactive_student_can_remove_existing_score() {
        for status in [Status::Leave, Status::Graduated] {
            let mut student = Student::new("Alice".into());
            assert!(student.add_score(80));
            student.change_status(status);

            assert_eq!(student.remove_last_score(), Some(80));
            assert!(student.scores().is_empty());
            assert_eq!(student.status(), status);
        }
    }

    #[test]
    fn rename_preserves_scores_and_status() {
        let mut student = Student::new("Alice".into());
        assert!(student.add_score(80));
        student.change_status(Status::Leave);

        student.rename("Carol".into());

        assert_eq!(student.name(), "Carol");
        assert_eq!(student.scores(), &[80]);
        assert_eq!(student.status(), Status::Leave);
    }

    #[test]
    fn negative_scores_are_supported() {
        let mut student = Student::new("Alice".into());
        assert!(student.add_score(-30));
        assert!(student.add_score(-10));

        assert_eq!(student.scores(), &[-30, -10]);
        assert_eq!(student.average(), -20.0);
        assert_eq!(student.highest_score(), Some(-10));
    }

    #[test]
    fn average_handles_extreme_scores_without_overflow() {
        for score in [i32::MAX, i32::MIN] {
            let mut student = Student::new("Alice".into());
            assert!(student.add_score(score));
            assert!(student.add_score(score));

            assert_eq!(student.average(), f64::from(score));
        }
    }

    #[test]
    fn average_preserves_fractional_part() {
        let mut student = Student::new("Alice".into());
        for score in [1, 2, 2] {
            assert!(student.add_score(score));
        }

        let expected = 5.0 / 3.0;

        assert!((student.average() - expected).abs() < 1e-9);
    }
}
