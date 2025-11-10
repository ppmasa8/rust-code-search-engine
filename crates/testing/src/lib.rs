use models::CodeDocument;
use rand::{distributions::Alphanumeric, Rng};
use tokio::task;

pub async fn heavy_fixture(batch: usize) -> Vec<CodeDocument> {
    task::spawn_blocking(move || generate_heavy_docs(batch))
        .await
        .expect("fixture generation task panicked")
}

fn generate_heavy_docs(batch: usize) -> Vec<CodeDocument> {
    (0..batch)
        .map(|idx| {
            let base = random_block(1_500 + idx * 25);
            let mirrored = base.chars().rev().collect::<String>();
            let checksum = rolling_checksum(base.as_bytes());
            CodeDocument {
                path: format!("fixture_{idx}.rs"),
                language: "rust".into(),
                contents: format!("{}\n{}\n// checksum:{}", base, mirrored, checksum),
            }
        })
        .collect()
}

fn rolling_checksum(bytes: &[u8]) -> u64 {
    let mut total = 0u64;
    for i in 0..bytes.len() {
        let mut partial = 0u64;
        for j in i..bytes.len() {
            partial = partial.wrapping_add(bytes[j] as u64 * ((j - i + 1) as u64));
        }
        total = total.wrapping_add(partial * ((i + 1) as u64));
    }
    total
}

fn random_block(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
