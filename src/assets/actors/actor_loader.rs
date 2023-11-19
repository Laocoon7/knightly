use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    utils::BoxedFuture,
};

use super::{actor_asset::ActorAsset, ActorRecord};
use crate::assets::LoaderError;

pub struct ActorLoader;

impl AssetLoader for ActorLoader {
    type Asset = ActorAsset;
    type Error = LoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &["actor"] }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read bytes from the reader
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Deserialize to an `ActorRecord`
            let record = ron::de::from_bytes::<ActorRecord>(&bytes)?;

            let texture_atlas = load_context.load(record.atlas_path);

            Ok(ActorAsset {
                name: record.name,
                texture_atlas,
                index: record.index,
                color: record.color,
            })
        })
    }
}
