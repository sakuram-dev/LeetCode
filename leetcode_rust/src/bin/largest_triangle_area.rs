// 812. Largest Triangle Area

fn main() {
    println!("{}", Solution::largest_triangle_area(vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]]));
}

struct Solution;

impl Solution {
    // caliculate the area of a triangle
    pub fn area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        0.5 * ((p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])).abs() as f64
    }

    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0.0;
        for i in 0..points.len() {
            for j in i+1..points.len() {
                for k in j+1..points.len() {
                    let area = Self::area(&points[i], &points[j], &points[k]);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }
        max_area
    }
}