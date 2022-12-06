// Reference code found here: https://paulkernfeld.com/2018/07/01/logistic-regression-in-rust.html

use ndarray::{Array1, Array2};

// This value is from scipy.optimize
// https://docs.scipy.org/doc/scipy/reference/optimize.minimize-lbfgsb.html
const FTOL: f64 = 2.220446049250313e-09;

const ARMIJO_GOLDSTEIN_CONTROL: f64 = 0.5;

    fn norm_l2(a_s: &Array1<f64>) -> f64 {
        return a_s.fold(0.0, |b, a| b + a * a);
    }

fn minimize_inner(beta_init: Array1<f64>, x: &Array2<f64>, y: &Array1<f64>, epsilon: f64) 
    -> Result<Array1<f64>, &'static str> {
    let mut beta = beta_init;

    let (mut prev_loss, mut prev_gradient) = loss_gradient(&beta, x, y);

    loop {
        beta.scaled_add(-epsilon, &prev_gradient);
        let (loss, gradient) = loss_gradient(&beta, x, y);

        let expected_decrease = epsilon * norm_l2(&gradient);
        let actual_decrease = prev_loss - loss;
        if actual_decrease < expected_decrease * ARMIJO_GOLDSTEIN_CONTROL {
            return Err("Armijo-Goldstein condition failed");
        }
        if actual_decrease < FTOL {
            return Ok(beta);
        }
        prev_loss = loss;
        prev_gradient = gradient;
    }
}

fn minimize(beta_init: Array1<f64>, x: &Array2<f64>, y: &Array1<f64>) -> Array1<f64> {
    for i in 0..20 {
        let epsilon = 2.0_f64.powi(-i);

        let beta_hat = minimize_inner(beta_init.clone(), x, y, epsilon);
        if beta_hat.is_ok() {
            return beta_hat.unwrap();
        }
    }
    panic!("Even a very small value of epsilon didn't work :(");
}

fn predict(beta_hat: &Array1<f64>, x: &Array2<f64>) -> Array1<f64> {
    return x.dot(beta_hat).t().map(|a| 1.0 / (1.0 + (-a).exp()));
}

fn loss_gradient(beta: &Array1<f64>, x: &Array2<f64>, y: &Array1<f64>) -> (f64, Array1<f64>) {
    let yhats = predict(beta, x);
    let loss = -y.iter()
        .zip(yhats.iter())
        .map(|(y, yhat)| y * yhat.ln() + (1.0 - y) * (1.0 - yhat).ln())
        .sum::<f64>();
    let gradient = (yhats - y).dot(x);
    return (loss, gradient);
}

pub fn logistic_regression(x: &Array2<f64>, y: &Array1<f64>) -> Array1<f64> {
    let (_n_data_points, n_features) = x.dim();
    let beta_hat = Array1::zeros(n_features);
    return minimize(beta_hat, x, y);
}