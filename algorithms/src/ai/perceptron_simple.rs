/// Aplica los pesos a los valores de entrada y devuelve la evaluación
/// junto con los nuevos pesos.
pub fn apply_weights(w: &Vec<f64>, x: &Vec<f64>, y: f64, n: &Vec<f64>) -> (f64, Vec<f64>) {
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += w[i] * x[i];
    }

    let diff = y - sum;
    let mut new_w = vec![0.0; w.len()];
    for i in 0..w.len() {
        new_w[i] = w[i] + n[i] * diff * x[i];
    }

    let heaviside = |x: f64| -> f64 {
        if x >= 0.0 {
            1.0
        } else {
            0.0
        }
    };

    (heaviside(sum), new_w)
}

/// Entrena una neurona simple para que aprenda la función AND.
pub fn entrenamiento() {
    let mut w = vec![0.0; 3];
    let x = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
    ];

    let y = vec![0.0, 0.0, 0.0, 1.0];
    let n = 0.1;
    let epochs = 10;

    for _ in 0..epochs {
        for i in 0..x.len() {
            let (_y, new_w) = apply_weights(&w, &x[i], y[i], &vec![n; 3]);
            w = new_w;
        }
    }

    println!("Pesos finales: {:?}", w);

    for i in 0..x.len() {
        let (y_res, _) = apply_weights(&w, &x[i], y[i], &vec![n; 3]);
        println!(
            "{} AND {} = {} (expected {})",
            x[i][0], x[i][1], y_res, y[i]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_weights() {
        let w = vec![0.0, 0.0, 0.0];
        let x = vec![0.0, 0.0, 1.0];
        let y = 0.0;
        let n = vec![0.1, 0.1, 0.1];

        let (y_res, new_w) = apply_weights(&w, &x, y, &n);
        assert_eq!(y_res, 1.0);
        assert_eq!(new_w, vec![0.0, 0.0, 0.0]);
    }
}
