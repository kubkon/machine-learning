use vector::Vector;
use linear_regression::LinearRegression;

#[test]
fn test_linear_regression() {
    let xs = [1.0,2.0,3.0];
    let ys = [1.0,2.0,3.0];
    let mut model = LinearRegression::new(0.1, 1e-12);
    model.fit(&xs[], &ys[]);
    assert_almost_eq!(model.params.get(0), 0.0, 1e-3);
    assert_almost_eq!(model.params.get(1), 1.0, 1e-3);
}

#[test]
fn test_vector_addition() {
    let v1 = Vector::from_slice(&[1.0,2.0,3.0]);
    let v2 = Vector::from_slice(&[3.0,4.0,5.0]);
    let exp = Vector::from_slice(&[4.0,6.0,8.0]);
    assert_eq!(v1.add(&v2).unwrap(), exp);

    let v3 = Vector::zeros(2);
    let v4 = Vector::zeros(3);
    assert_eq!(v3.add(&v4), None);
}

#[test]
fn test_vector_scalar_multiplication() {
    let v1 = Vector::ones(3);
    let m = 2.0;
    let exp = Vector::from_slice(&[m,m,m]);
    assert_eq!(v1.scalar_mul(m), exp);
}

#[test]
fn test_vector_multiplication() {
    let v1 = Vector::ones(3);
    let v2 = Vector::from_slice(&[1.0,2.0,3.0]);
    assert_eq!(v1.mul(&v2).unwrap(), 6.0);
}

#[test]
fn test_vector_subtraction() {
    let v1 = Vector::zeros(3);
    let v2 = Vector::from_slice(&[1.0,2.0,3.0]);
    let exp = Vector::from_slice(&[-1.0,-2.0,-3.0]);
    assert_eq!(v1.sub(&v2).unwrap(), exp);
}

#[test]
fn test_vector_iter() {
    let xs = [1.0,2.0,3.0,4.0];
    let v1 = Vector::from_slice(&xs);
    for (i, &v) in v1.iter().enumerate() {
        assert_eq!(xs[i], v);
    }
}

#[test]
fn test_vector_as_slice() {
    let xs = [1.0,2.0,3.0];
    assert_eq!(Vector::from_slice(&xs).as_slice(), xs);
}
