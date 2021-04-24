use tokio_compat_02::FutureExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    gudlink::run().compat().await
}
