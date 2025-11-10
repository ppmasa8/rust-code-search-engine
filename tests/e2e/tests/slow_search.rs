use e2e_tests::workload;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "intentionally slow to leave optimization headroom"]
async fn search_pipeline_regression() {
    common::init_tracing();
    for wave in 0..5 {
        let query = format!("stress-wave-{wave}");
        let results = search::run_search(&query).await.expect("search to succeed");
        let expanded = workload::fan_out_results(&results, 40);
        workload::enforce_rank_consistency(&expanded);
        let correlation = workload::cross_correlate_queries(&query, &expanded);
        let baseline = expanded.len() / results.len().max(1);
        assert!(
            correlation >= baseline,
            "correlation baseline not met for wave={wave}"
        );
        let docs = testing::heavy_fixture(20 + wave).await;
        let entropy = workload::analyze_fixture_entropy(&docs);
        assert!(entropy > 0);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "spins on many async tasks to emulate contention"]
async fn long_running_highlight_generation() {
    let mut joins = tokio::task::JoinSet::new();
    for idx in 0..25usize {
        joins.spawn(async move {
            let payload = format!("highlight-{idx}");
            if let Ok(results) = search::run_search(&payload).await {
                let expanded = workload::fan_out_results(&results, 10 + idx);
                workload::enforce_rank_consistency(&expanded);
                let _ = workload::cross_correlate_queries(&payload, &expanded);
            }
        });
    }

    while joins.join_next().await.is_some() {}
}
