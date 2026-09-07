// PROBLEM: Show the ID of the student who obtained the highest grade in the programming fundamentals midterm.
// We define a basic struct to represent the Student
pub struct Student {
    pub id: String,
    pub grade: f64,
}

pub fn student_with_highest_grade(students: &[Student]) -> Option<String> {
    students.iter()
        .max_by(|a, b| a.grade.partial_cmp(&b.grade).unwrap_or(std::cmp::Ordering::Equal))
        .map(|student| student.id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_with_highest_grade() {
        let list = vec![
            Student {
                id: "A1".to_string(),
                grade: 4.2,
            },
            Student {
                id: "B2".to_string(),
                grade: 4.8,
            },
        ];
        assert_eq!(student_with_highest_grade(&list), Some("B2".to_string()));
    }
}