use e2e_tests::workload;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "simulates embedding drift analysis"]
async fn embedding_vector_health_check() {
    let mut checksum = 0.0f32;
    for round in 0..50usize {
        let payload = format!("fn drift_round_{round}() {{}}");
        let vector = embedding::encode(payload.as_bytes());
        checksum += vector.iter().sum::<f32>();
        let similarity = workload::vector_similarity_score(&vector);
        assert!(similarity.is_finite());
    }
    assert!(checksum.is_finite());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "loops over fixture building and sleeps"]
async fn embedding_fixture_alignment() {
    for batch in [5usize, 15, 25, 35, 45] {
        let docs = testing::heavy_fixture(batch).await;
        assert_eq!(docs.len(), batch);
        let merge_cost = workload::simulate_merge_cost(&docs);
        let entropy = workload::analyze_fixture_entropy(&docs);
        assert!(merge_cost > 0 && entropy > 0);
    }
}
