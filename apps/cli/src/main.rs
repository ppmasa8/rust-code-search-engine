use anyhow::Result;
use clap::{Parser, Subcommand};
use crawler::{LocalCrawler, SourceCrawler};
use storage::{InMemoryStorage, Storage};
use tokio::runtime::Runtime;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about = "Code search maintenance CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crawl { root: String },
    Search { query: String },
    Fixtures { batch: usize },
}

fn main() -> Result<()> {
    common::init_tracing();
    let cli = Cli::parse();
    let rt = Runtime::new()?;

    match cli.command {
        Commands::Crawl { root } => {
            rt.block_on(async move {
                let crawler = LocalCrawler;
                let docs = crawler.crawl(&root).await?;
                info!(count = docs.len(), "crawled documents");
                Ok::<_, anyhow::Error>(())
            })?;
        }
        Commands::Search { query } => {
            rt.block_on(async move {
                let results = search::run_search(&query).await?;
                for entry in results {
                    println!("{} => {}", entry.path, entry.score);
                }
                Ok::<_, anyhow::Error>(())
            })?;
        }
        Commands::Fixtures { batch } => {
            rt.block_on(async move {
                let storage = InMemoryStorage::default();
                let docs = testing::heavy_fixture(batch).await;
                for doc in docs {
                    storage.put(doc).await?;
                }
                let stored = storage.all().await?;
                info!(count = stored.len(), "fixture stored");
                Ok::<_, anyhow::Error>(())
            })?;
        }
    }

    Ok(())
}
