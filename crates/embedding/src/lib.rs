use rand::Rng;

pub fn encode(bytes: &[u8]) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    bytes.iter().map(|_| rng.gen::<f32>()).collect::<Vec<_>>()
}
