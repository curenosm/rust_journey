use std::collections::HashMap;

pub fn lda() {
    /// First import the csv package
    /// Define a function to perform the lda algorithm
    // Load the data from the csv file
    let mut rdr = csv::Reader::from_path("datasets/iris.csv").unwrap();

    // Create a vector to store the data
    let mut data: Vec<Vec<f64>> = Vec::new();

    // Iterate over the records and store them
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<f64> = Vec::new();
        for i in 0..record.len() - 1 {
            let value = record[i].parse::<f64>().unwrap();
            row.push(value);
        }
        data.push(row);
    }

    // Now we need to create the labels
    let mut labels: Vec<String> = Vec::new();

    // Iterate over the records and store them
    for result in rdr.records() {
        let record = result.unwrap();
        let label = record[record.len() - 1].to_string();
        labels.push(label);
    }

    // Print the prediction
    println!("Prediction: {}", 1);
}

pub fn calcular_media(
    data: &Vec<Vec<f64>>,
    y: &Vec<String>,
    clase: &str,
) -> HashMap<String, Vec<f64>> {
    //! Función para calcular la media de cada clase

    // Create a new empty HashMap
    let mut medias: HashMap<String, Vec<f64>> = HashMap::new();

    // Iterate over the records and store them
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

    // Now we need to divide by the number of elements
    for (key, value) in medias.iter_mut() {
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
    // Calculamos los vectores medios dimensionales para las diferentes clases del conjunto de datos

    // Calculamos las matrices de dispersión (entre clases y dentro de clases)

    // Dentro de cada clase
    // Create a new empty HashMap
    let mut sw: Vec<Vec<f64>> = Vec::new();

    // Iterate over the records and store them
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

pub fn calcular_sb(data: &Vec<Vec<f64>>, medias: &HashMap<String, Vec<f64>>) -> Vec<Vec<f64>> {
    /// Variable del bias, convolutional
    // Create a new empty HashMap
    let mut st: Vec<Vec<f64>> = Vec::new();

    // Dispersión total, obtenemos la matriz de covarianza de la transpuesta de X
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
    // Create a new empty HashMap
    let mut eigen_values: Vec<f64> = Vec::new();
    let mut eigen_vectors: Vec<Vec<f64>> = Vec::new();

    // Calculamos la matriz de proyección
    for i in 0..sw.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..sw.len() {
            row.push(sw[i][j] / st[i][j]);
        }
        eigen_vectors.push(row);
    }

    (eigen_values, eigen_vectors)
}

pub fn calculate_inverse_matrix(sw: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    /// We have to calculate the eigen values and eigen vectors
    /// of the matrix sw^-1 * st
    let eigen_res = eigen(sw, sw);

    let idx = eigen_res.0.iter().position(|&r| r == 0.0).unwrap();

    // Sort it from the highest to the lowest
    let mut eigen_values = eigen_res.0.clone();
    eigen_values.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Create a new empty HashMap
    let matriz_inversa: Vec<Vec<f64>> = Vec::new();

    matriz_inversa
}

pub fn transform_data(data: &Vec<Vec<f64>>, projection_matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Create a new empty HashMap
    let mut transformed_data: Vec<Vec<f64>> = Vec::new();

    // Calculamos la matriz de proyección // Dot product
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

pub fn calculate_projection_matrix(sw: &Vec<Vec<f64>>, st: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    /// We have to calculate the eigen values and eigen vectors
    /// of the matrix sw^-1 * st
    let eigen_res = eigen(sw, st);

    // Create a new empty HashMap
    let mut projection_matrix: Vec<Vec<f64>> = Vec::new();

    projection_matrix
}
