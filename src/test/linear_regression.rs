use Vector;
use LinearRegression;

#[test]
fn test_univariate_linear_regression() {
    let xs = [Vector::from_slice(&[1.0]),
              Vector::from_slice(&[2.0]),
              Vector::from_slice(&[3.0])];
    let ys = Vector::from_slice(&[1.0,2.0,3.0]);
    let n = xs[0].len() + 1;
    let mut model = LinearRegression::new(0.1, &Vector::zeros(n));
    model.tolerance = 1e-8;
    model.fit(&xs[], &ys);
    assert_almost_eq!(model.params.get(0), 0.0, 1e-6);
    assert_almost_eq!(model.params.get(1), 1.0, 1e-6);
}

#[test]
fn test_multivariate_linear_regression() {
    let xs = [Vector::from_slice(&[1.0, 1.0]),
              Vector::from_slice(&[2.0, 2.0]),
              Vector::from_slice(&[3.0, 3.0]),
              Vector::from_slice(&[4.0, 4.0])];
    let ys = Vector::from_slice(&[1.0,2.0,3.0,4.0]);
    let n = xs[0].len() + 1;
    let mut model = LinearRegression::new(0.1, &Vector::zeros(n));
    model.tolerance = 1e-8;
    model.fit(&xs[], &ys);
    assert_almost_eq!(model.params.get(0), 0.0, 1e-6);
    assert_almost_eq!(model.params.get(1), 0.5, 1e-6);
    assert_almost_eq!(model.params.get(2), 0.5, 1e-6);
}
