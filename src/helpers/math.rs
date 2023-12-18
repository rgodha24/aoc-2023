use super::Point;

/// shoelace formula for the area of a polygon given its vertices
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

pub fn lcm(nums: impl Iterator<Item = usize>) -> usize {
    nums.fold(1, num::integer::lcm)
}

pub fn gcd(nums: impl Iterator<Item = usize>) -> usize {
    nums.fold(0, num::integer::gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd([33, 99, 132].into_iter()), 33);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm([33, 99, 132].into_iter()), 396);
    }
}
