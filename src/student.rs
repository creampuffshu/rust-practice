use core::fmt;

use crate::{summary::Summary};

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
        let sum: i32 = self.scores.iter().sum();
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
