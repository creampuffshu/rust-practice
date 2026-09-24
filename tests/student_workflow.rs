use hello_rust::classroom::{Classroom, CommandError};
use hello_rust::student::Status;
use hello_rust::summary::Summary;

#[test]
fn student_lifecycle_through_commands() {
    let mut classroom = Classroom::new();

    // 학생 등록
    classroom.execute("add Alice").unwrap();
    classroom.execute("add Bob").unwrap();

    assert_eq!(classroom.student_count(), 2);
    assert_eq!(classroom.active_student_count(), 2);

    // 성적 입력
    classroom.execute("score Alice 80").unwrap();
    classroom.execute("score Alice 90").unwrap();

    let alice = classroom.find_student("Alice").unwrap();
    assert_eq!(alice.scores(), &[80, 90]);
    assert!((alice.average() - 85.0).abs() < 1e-9);
    assert_eq!(alice.highest_score(), Some(90));

    // 휴학
    classroom.execute("status Alice leave").unwrap();

    assert_eq!(classroom.active_student_count(), 1);
    assert_eq!(
        classroom.find_student("Alice").unwrap().status(),
        Status::Leave
    );

    // 휴학 중 성적 입력은 실패하고 기존 성적은 유지
    assert_eq!(
        classroom.execute("score Alice 100"),
        Err(CommandError::CannotAddScore)
    );
    assert_eq!(
        classroom.find_student("Alice").unwrap().scores(),
        &[80, 90]
    );

    // 복학 후 성적 입력
    classroom.execute("status Alice active").unwrap();

    assert_eq!(classroom.active_student_count(), 2);

    classroom.execute("score Alice 100").unwrap();

    let alice = classroom.find_student("Alice").unwrap();
    assert_eq!(alice.status(), Status::Active);
    assert_eq!(alice.scores(), &[80, 90, 100]);
    assert!((alice.average() - 90.0).abs() < 1e-9);
    assert_eq!(alice.highest_score(), Some(100));

    // 다른 학생 삭제
    classroom.execute("remove Bob").unwrap();

    assert_eq!(classroom.student_count(), 1);
    assert_eq!(classroom.active_student_count(), 1);
    assert!(classroom.find_student("Bob").is_none());

    // 최종 요약
    assert_eq!(
        classroom.summary(),
        "학생 수: 1 | 재학생 수: 1"
    );
}