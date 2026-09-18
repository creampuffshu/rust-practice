mod student;
mod classroom;
use student::Status;
use classroom::Classroom;


fn main() {
    let mut classroom = Classroom::new();

    println!("{}", classroom.add_student("Alice".to_string()));   // true
    println!("{}", classroom.add_student("Bob".to_string()));     // true
    println!("{}", classroom.add_student("Charlie".to_string())); // true
    println!("{}", classroom.add_student("Alice".to_string()));   // false

    if let Some(student) = classroom.find_student_mut("Alice") {
        student.add_score(80);
        student.add_score(90);
    }

    if let Some(student) = classroom.find_student_mut("Bob"){
        student.change_status(Status::Leave);
        if student.add_score(100) == false {
            println!("점수 추가 실패~~");
        }
    }

    if let Some(student) = classroom.find_student("Alice") {
        println!("평균: {}",student.average());
    }

    classroom.remove_student("Bob");
    if classroom.find_student("David").is_none() {
        println!("David 없음!!");
    }

    classroom.print_all();

    println!(
        "이름 변경: {}",
        classroom.rename_student("Alice", "Alicia".to_string())
    );

    println!("학생 수: {}", classroom.student_count());
    println!("재학생 수: {}", classroom.active_student_count());

    println!(
        "Alice 존재: {}",
        classroom.find_student("Alice").is_some()
    );

    println!(
        "Alicia 존재: {}",
        classroom.find_student("Alicia").is_some()
    );

    classroom.print_all();
}