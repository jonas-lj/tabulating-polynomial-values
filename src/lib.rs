use std::iter::successors;
use std::ops::{Add, Mul, Sub};

/// This iterator yields evaluations of a polynomial at points in an arithmetic progression, e.g., x0, x0+h, x0+2h, ...
/// This is generally faster than just evaluating each point naively when evaluating more points than the degree of the polynomial.
///
/// The iterator is in principle infinite, but it's up to the caller to ensure that no overflow occurs for the given type.
///
/// The algorithm is from section 4.6.4 in Knuth's "Art of Computer Programming" where it is called "Tabulating polynomial values".
pub struct PolynomialEvaluator<C> {
    state: Vec<C>,
    first: bool,
    input: C,
    step: C,
}

impl<C: Clone> PolynomialEvaluator<C>
where
    for<'a> C: Mul<&'a C, Output = C>,
    for<'a> C: Add<&'a C, Output = C>,
    for<'a> &'a C: Add<&'a C, Output = C>,
    for<'a> &'a C: Sub<&'a C, Output = C>,
{
    /// Construct a new iterator which yields the evaluations of the polynomial defined by the `coefficients`
    /// on the points `initial`, `initial + step`, `initial + 2*step`, ... .
    pub fn new(coefficients: &[C], initial: C, step: C) -> Self {
        // Compute initial values (see exercise 7 in 4.6.4 of TAOCP)
        let mut state = successors(Some(initial.clone()), |x| Some(x + &step))
            .take(coefficients.len())
            .map(|x| evaluate(coefficients, &x))
            .collect::<Vec<_>>();
        for k in 1..coefficients.len() {
            for j in (k..coefficients.len()).rev() {
                state[j] = &state[j] - &state[j - 1];
            }
        }
        Self {
            state,
            first: true,
            input: initial,
            step,
        }
    }
}

impl<C: Clone> Iterator for PolynomialEvaluator<C>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    type Item = (C, C);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            for j in 0..self.state.len() - 1 {
                self.state[j] = &self.state[j] + &self.state[j + 1]
            }
            self.input = &self.input + &self.step;
        }
        Some((self.input.clone(), self.state[0].clone()))
    }
}

/// Evaluate a polynomial using Horner's method.
/// Panics if `coefficients` is empty.
fn evaluate<C: Clone>(coefficients: &[C], input: &C) -> C
where
    for<'a> C: Mul<&'a C, Output = C>,
    for<'a> C: Add<&'a C, Output = C>,
{
    assert!(coefficients.len() > 0);
    if coefficients.len() == 1 {
        return coefficients[0].clone();
    }
    coefficients
        .iter()
        .rev()
        .skip(1)
        .fold(coefficients.last().unwrap().clone(), |sum, coefficient| {
            sum * input + coefficient
        })
}

#[test]
fn test_evaluation() {
    let polynomial = [1, 2, 3];
    let y = evaluate(&polynomial, &7);
    assert_eq!(y, 1 + 2 * 7 + 3 * 7 * 7);
}

#[test]
fn test_polynomial_evaluator() {
    let polynomial = [1, 2, 3];
    let evaluator = PolynomialEvaluator::new(&polynomial, 0, 1);
    for (x, y) in evaluator.take(100) {
        assert_eq!(y, evaluate(&polynomial, &x))
    }

    let evaluator = PolynomialEvaluator::new(&polynomial, 7, 5);
    for (x, y) in evaluator.take(10) {
        assert_eq!(y, evaluate(&polynomial, &x))
    }
}
