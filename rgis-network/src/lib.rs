#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use futures_util::StreamExt;
use std::io;
pub struct FetchedFile {
    pub name: String,
    pub bytes: bytes::Bytes,
    pub crs_epsg_code: u16,
}

pub struct NetworkFetchJob {
    pub url: String,
    pub crs_epsg_code: u16,
    pub name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

impl bevy_jobs::Job for NetworkFetchJob {
    type Outcome = Result<FetchedFile, Error>;

    fn name(&self) -> String {
        format!("Fetching '{}'", self.name)
    }

    fn perform(self, ctx: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let fetch = async {
                let response = reqwest::get(self.url).await?;
                let total_size = response.content_length().unwrap_or(0);
                let mut bytes_stream = response.bytes_stream();
                let mut bytes = Vec::<u8>::with_capacity(total_size as usize);

                while let Some(bytes_chunk) = bytes_stream.next().await {
                    let mut bytes_chunk = Vec::from(bytes_chunk?);
                    bytes.append(&mut bytes_chunk);
                    if total_size > 0 {
                        let _ = ctx
                            .send_progress((bytes.len() / total_size as usize) as u8)
                            .await;
                    }
                }

                Ok(FetchedFile {
                    bytes: bytes::Bytes::from(bytes),
                    crs_epsg_code: self.crs_epsg_code,
                    name: self.name,
                })
            };
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            runtime.block_on(fetch)
        })
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}
