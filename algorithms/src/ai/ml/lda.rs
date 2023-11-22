use std::collections::HashMap;

/// Function to perform a demonstration of Linear Discriminant Analysis
pub fn lda() {
    let mut rdr = csv::Reader::from_path("datasets/iris.csv").unwrap();

    let mut data: Vec<Vec<f64>> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<f64> = Vec::new();
        for i in 0..record.len() - 1 {
            let value = record[i].parse::<f64>().unwrap();
            row.push(value);
        }
        data.push(row);
    }

    let mut labels: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let label = record[record.len() - 1].to_string();
        labels.push(label);
    }

    println!("Prediction: {}", 1);
}

/// Función para calcular la media de cada clase
pub fn calcular_media(
    data: &Vec<Vec<f64>>,
    y: &Vec<String>,
    _clase: &str,
) -> HashMap<String, Vec<f64>> {
    let mut medias: HashMap<String, Vec<f64>> = HashMap::new();

    for i in 0..data.len() {
        let record = &data[i];
        let label = &y[i];
        if medias.contains_key(label) {
            let mut media = medias.get(label).unwrap().clone();
            for j in 0..record.len() {
                media[j] += record[j];
            }
            medias.insert(label.to_string(), media);
        } else {
            medias.insert(label.to_string(), record.clone());
        }
    }

    for (_key, value) in medias.iter_mut() {
        for i in 0..value.len() {
            value[i] /= data.len() as f64;
        }
    }

    medias
}

pub fn calcular_sw(
    data: &Vec<Vec<f64>>,
    y: &Vec<String>,
    medias: &HashMap<String, Vec<f64>>,
) -> Vec<Vec<f64>> {
    let mut sw: Vec<Vec<f64>> = Vec::new();

    for i in 0..data.len() {
        let record = &data[i];
        let label = &y[i];
        let media = medias.get(label).unwrap();
        let mut row: Vec<f64> = Vec::new();
        for j in 0..record.len() {
            row.push((record[j] - media[j]).powi(2));
        }
        sw.push(row);
    }

    sw
}

/// Variable del bias, convolutional
pub fn calcular_sb(data: &Vec<Vec<f64>>, medias: &HashMap<String, Vec<f64>>) -> Vec<Vec<f64>> {
    let mut st: Vec<Vec<f64>> = Vec::new();

    for i in 0..data[0].len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..data[0].len() {
            let mut sum = 0.0;
            for k in 0..data.len() {
                sum += (data[k][i] - medias.get(&k.to_string()).unwrap()[i])
                    * (data[k][j] - medias.get(&k.to_string()).unwrap()[j]);
            }
            row.push(sum);
        }
        st.push(row);
    }

    st
}

pub fn eigen(sw: &Vec<Vec<f64>>, st: &Vec<Vec<f64>>) -> (Vec<f64>, Vec<Vec<f64>>) {
    let eigen_values: Vec<f64> = Vec::new();
    let mut eigen_vectors: Vec<Vec<f64>> = Vec::new();

    for i in 0..sw.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..sw.len() {
            row.push(sw[i][j] / st[i][j]);
        }
        eigen_vectors.push(row);
    }

    (eigen_values, eigen_vectors)
}

/// Function to calculate the eigen values and eigen vectors
/// of the matrix sw^-1 * st
pub fn calculate_inverse_matrix(sw: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let _eigen_res = eigen(sw, sw);

    let _idx = _eigen_res.0.iter().position(|&r| r == 0.0).unwrap();

    let mut eigen_values = _eigen_res.0.clone();
    eigen_values.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let matriz_inversa: Vec<Vec<f64>> = Vec::new();

    matriz_inversa
}

pub fn transform_data(data: &Vec<Vec<f64>>, projection_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut transformed_data: Vec<Vec<f64>> = Vec::new();

    for i in 0..data.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..projection_matrix.len() {
            let mut sum = 0.0;
            for k in 0..data[0].len() {
                sum += data[i][k] * projection_matrix[j][k];
            }
            row.push(sum);
        }
        transformed_data.push(row);
    }

    transformed_data
}

/// We have to calculate the eigen values and eigen vectors
/// of the matrix sw^-1 * st
pub fn calculate_projection_matrix(sw: &Vec<Vec<f64>>, st: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let _eigen_res = eigen(sw, st);
    let projection_matrix: Vec<Vec<f64>> = Vec::new();

    projection_matrix
}
