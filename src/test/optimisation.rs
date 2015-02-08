use std::num::Float;
use Vector;
use core::optimisation::gradient_descent;

#[test]
fn test_gradient_descent() {
    let init = Vector::zeros(2);
    let func = |&: params: &Vector| -> f64 {
        1.0
    };
    // Gradient of Booth's function
    let gradient = |&: params: &Vector| -> Vector {
        let p1 = params.get(0);
        let p2 = params.get(1);
        let d1 = 2.0*(p1 + 2.0*p2 - 7.0) + 4.0*(2.0*p1 + p2 - 5.0);
        let d2 = 4.0*(p1 + 2.0*p2 - 7.0) + 2.0*(2.0*p1 + p2 - 5.0);
        Vector::from_slice(&[d1, d2])
    };
    let given = gradient_descent(0.1, 1e-6, &init, func, gradient);
    let exp = Vector::from_slice(&[1.0,3.0]);
    assert_almost_eq!(exp.get(0), given.get(0), 1e-6);
    assert_almost_eq!(exp.get(1), given.get(1), 1e-6);
}

