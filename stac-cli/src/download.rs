use crate::Result;
use serde::Serialize;
use stac::{Assets, Href, Links};
use stac_async::Downloader;
use std::path::Path;

pub async fn download<A>(assets: A, directory: impl AsRef<Path>, _: bool) -> Result<()>
where
    A: Assets + Href + Links + Serialize + Clone,
{
    let downloader = Downloader::new(assets)?;
    downloader.download(directory).await?;
    Ok(())
}