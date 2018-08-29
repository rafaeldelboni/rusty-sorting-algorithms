use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub fn random_vector(vector_size: isize) -> Vec<isize> {
    let mut generated_vector: Vec<isize> = Vec::new();
    for _ in 0..vector_size {
        let generated_number = thread_rng().gen_range(0, vector_size);
        generated_vector.push(generated_number);
    }
    generated_vector
}
