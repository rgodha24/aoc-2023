use super::Point;

pub fn shoelace(points: &[Point]) -> isize {
    let mut sum = 0;
    let len = points.len() as isize;
    for i in 0..(points.len() as isize) {
        let prev = (i - 1).rem_euclid(len) as usize;
        let next = (i + 1).rem_euclid(len) as usize;
        let x = points[i as usize].x;

        sum += (points[next].y - points[prev].y) * x;
    }

    assert!(sum % 2 == 0);
    (sum / 2).abs()
}

/// picks theorem gives the inside coordinates of a polygon given the area and the boundaries
pub fn picks(area: isize, boundaries: isize) -> isize {
    area - boundaries / 2 + 1
}
