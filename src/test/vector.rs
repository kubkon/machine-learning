use Vector;

#[test]
fn test_vector_addition() {
    let v1 = Vector::from_slice(&[1.0,2.0,3.0]);
    let v2 = Vector::from_slice(&[3.0,4.0,5.0]);
    let exp = Vector::from_slice(&[4.0,6.0,8.0]);
    assert_eq!(v1.add(&v2), exp);
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
    assert_eq!(v1.mul(&v2), 6.0);
}

#[test]
fn test_vector_subtraction() {
    let v1 = Vector::zeros(3);
    let v2 = Vector::from_slice(&[1.0,2.0,3.0]);
    let exp = Vector::from_slice(&[-1.0,-2.0,-3.0]);
    assert_eq!(v1.sub(&v2), exp);
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
