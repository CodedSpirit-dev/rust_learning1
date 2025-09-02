// operations/src/lib.rs

/// Adds two numbers.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts `b` from `a`.
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers.
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides `a` by `b`. Returns `None` if `b` is zero.
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4.0, 3.0), 12.0);
    }

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(6.0, 2.0), Some(3.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(1.0, 0.0), None);
    }
}