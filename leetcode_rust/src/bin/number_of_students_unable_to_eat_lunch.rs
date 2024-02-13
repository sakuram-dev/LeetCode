// 1700. Number of Students Unable to Eat Lunch
fn main() {
    let students = vec![1,1,1,0,0,1];
    let sandwiches = vec![1,0,0,0,1,1];
    println!("{}", Solution::count_students(students, sandwiches));
}

struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students;
        let mut sandwiches = sandwiches;
        let mut i = 0;
        while i < students.len() {
            if students[0] == sandwiches[0] {
                students.remove(0);
                sandwiches.remove(0);
                i = 0;
            } else {
                let student = students.remove(0);
                students.push(student);
                i += 1;
            }
        }
        students.len() as i32
    }
}