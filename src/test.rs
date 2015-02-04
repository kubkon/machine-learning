use linear_regression::LinearRegression;

#[test]
fn test_linear_regression() {
    let xs = [1.0f64, 2.0, 3.0];
    let ys = [1.0f64, 2.0, 3.0];
    let mut model = LinearRegression::new(0.1, 1e-12);
    model.fit(&xs[], &ys[]);
    assert_almost_eq!(model.params[0], 0.0, 1e-3);
    assert_almost_eq!(model.params[1], 1.0, 1e-3);
}
