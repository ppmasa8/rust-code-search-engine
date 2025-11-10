use crawler::SourceCrawler;
use e2e_tests::workload;
use storage::Storage;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "hammer indexer with mocked documents"]
async fn indexer_handles_dense_batches() {
    let crawler = crawler::LocalCrawler;
    for round in 0..12 {
        let docs = crawler
            .crawl(&format!("/tmp/repo_{round}"))
            .await
            .expect("crawl to succeed");
        let count = indexer::add_documents(&docs).expect("index to accept batch");
        assert_eq!(count, docs.len());
        let merge_cost = workload::simulate_merge_cost(&docs);
        assert!(
            merge_cost >= docs.len().saturating_sub(1),
            "merge cost too small for round {round}"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "thrashes storage backend"]
async fn storage_snapshot_round_robin() {
    let store = storage::InMemoryStorage::default();
    for step in 0..40usize {
        let docs = testing::heavy_fixture(5 + (step % 5)).await;
        for doc in docs {
            store.put(doc).await.expect("put to succeed");
        }
        let snapshot = store.all().await.expect("snapshot to succeed");
        assert!(!snapshot.is_empty());
        let checksum = workload::verify_storage_snapshot(&snapshot);
        assert!(checksum > 0);
    }
}
