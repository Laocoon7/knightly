use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};

use super::AtlasRecord;
use crate::assets::LoaderError;

pub struct AtlasLoader;

impl AssetLoader for AtlasLoader {
    type Asset = TextureAtlas;
    type Error = LoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &["atlas"] }

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
            let record = ron::de::from_bytes::<AtlasRecord>(&bytes)?;

            let texture = load_context.load(record.texture_path);

            Ok(TextureAtlas::from_grid(
                texture,
                record.tile_size,
                record.columns,
                record.rows,
                record.padding,
                record.offset,
            ))
        })
    }
}
