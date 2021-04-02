use ibc::ics02_client::client_state::AnyClientState;

use crate::chain::Chain;
use crate::error;
use ibc::ics02_client::client_misbehaviour::AnyMisbehaviour;
use ibc::ics02_client::events::UpdateClient;

pub mod tendermint;

#[cfg(test)]
pub mod mock;

/// Defines a light block from the point of view of the relayer.
pub trait LightBlock<C: Chain>: Send + Sync {
    fn signed_header(&self) -> &C::Header;
}

/// Defines a client from the point of view of the relayer.
pub trait LightClient<C: Chain>: Send + Sync {
    /// Fetch a header from the chain at the given height and verify it
    fn verify(
        &mut self,
        trusted: ibc::Height,
        target: ibc::Height,
        client_state: &AnyClientState,
    ) -> Result<C::LightBlock, error::Error>;

    fn build_misbehaviour(
        &mut self,
        client_state: &AnyClientState,
        update: UpdateClient,
        latest_chain_height: ibc::Height,
    ) -> Result<Option<AnyMisbehaviour>, error::Error>;
    /// Fetch a header from the chain at the given height, without verifying it
    fn fetch(&mut self, height: ibc::Height) -> Result<C::LightBlock, error::Error>;
}
