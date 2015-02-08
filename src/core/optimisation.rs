use std::num::Float;
use Vector;

pub fn gradient_descent<'r, F>(step_size: f64,
                               tolerance: f64,
                               init_params: &Vector,
                               gradient_f: F) -> Vector<'r>
where F: Fn(&Vector) -> Vector<'r> {
    let mut params = init_params.clone();
    loop {
        let gradient = (gradient_f)(&params);
        let new_params = params.sub(&gradient.scalar_mul(step_size));
        let params_diff = new_params.sub(&params);
        params = new_params;
        if params_diff.mul(&params_diff).sqrt() <= tolerance {
            break;
        }
    }
    params
}

