use subxt::ClientBuilder;
use codec::{Decode, Encode};

#[derive(Clone, Copy, Decode, Debug, Encode, Eq, Ord, PartialEq, PartialOrd)]
pub enum DidEncryptionKey {
    /// An X25519 public key.
    X25519([u8; 32]),
}
#[derive(Clone, Copy, Decode, Debug, Encode, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id(u32);

#[subxt::subxt(runtime_metadata_path = "res/metadata.scale")]
pub mod kilt_peregrine {
    #[subxt(substitute_type = "polkadot_parachain::primitives::Id")]
    use crate::Id;
    #[subxt(substitute_type = "did::did_details::DidEncryptionKey")]
    use crate::DidEncryptionKey;
}
pub const END_POINT: &'static str = "wss://peregrine.kilt.io/";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url(END_POINT)
        .build()
        .await?
        .to_runtime_api::<kilt_peregrine::RuntimeApi<kilt_peregrine::DefaultConfig>>();

    let mut iter = api.storage().system().account_iter(None).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }
    Ok(())
}
