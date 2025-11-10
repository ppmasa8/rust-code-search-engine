use anyhow::Result;
use tokio::{
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    common::init_tracing();
    let mut jobs = JoinSet::new();

    for job_id in 0..5 {
        jobs.spawn(async move {
            sleep(Duration::from_millis(100 * job_id)).await;
            let query = format!("auto-job-{job_id}");
            let _ = search::run_search(&query).await;
            testing::heavy_fixture((job_id + 1) as usize).await;
            info!(job_id, "completed background job");
        });
    }

    while (jobs.join_next().await).is_some() {}
    Ok(())
}
