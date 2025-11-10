use e2e_tests::workload;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "boots fake api layers and sleeps"]
async fn api_handles_bursty_payloads() {
    common::init_tracing();
    for batch in 1..=10 {
        let req = contracts::SearchRequest {
            query: format!("fn batch_{batch}()"),
        };
        let response = simulate_api_call(req).await;
        workload::validate_api_response(&response);
        let docs = testing::heavy_fixture(batch + 5).await;
        let entropy = workload::analyze_fixture_entropy(&docs);
        assert!(entropy > 0);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "drives repeated fixture creation"]
async fn api_backpressure_window() {
    let mut pressure_score = 0usize;
    for cycle in 0..8usize {
        let docs = testing::heavy_fixture(15 + cycle).await;
        pressure_score = pressure_score.wrapping_add(workload::simulate_merge_cost(&docs));
        let response = simulate_api_call(contracts::SearchRequest {
            query: format!("cycle:{cycle}"),
        })
        .await;
        workload::validate_api_response(&response);
    }
    assert!(pressure_score > 0);
}

async fn simulate_api_call(request: contracts::SearchRequest) -> contracts::SearchResponse {
    let results = search::run_search(&request.query)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|item| contracts::SearchItem {
            path: item.path,
            score: item.score,
            snippet: item.highlight,
        })
        .collect();

    contracts::SearchResponse { results }
}
