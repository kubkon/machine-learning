use Vector;
use LinearRegression;

#[test]
fn test_linear_regression() {
    let xs = Vector::from_slice(&[1.0,2.0,3.0]);
    let ys = Vector::from_slice(&[1.0,2.0,3.0]);
    let mut model = LinearRegression::new(0.1, &Vector::zeros(2));
    model.tolerance = 1e-8;
    model.fit(&xs, &ys);
    assert_almost_eq!(model.params.get(0), 0.0, 1e-6);
    assert_almost_eq!(model.params.get(1), 1.0, 1e-6);
}
