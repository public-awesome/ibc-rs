use alloc::sync::Arc;

use abscissa_core::{Options, Runnable};
use tokio::runtime::Runtime as TokioRuntime;

use ibc::ics24_host::identifier::{ChainId, PortChannelId};
use ibc_proto::ibc::core::channel::v1::QueryChannelsRequest;
use ibc_relayer::chain::{Chain, CosmosSdkChain};

use crate::conclude::Output;
use crate::prelude::*;

#[derive(Clone, Command, Debug, Options)]
pub struct QueryChannelsCmd {
    #[options(free, required, help = "identifier of the chain to query")]
    chain_id: ChainId,
}

// hermes query channels ibc-0
impl Runnable for QueryChannelsCmd {
    fn run(&self) {
        let config = app_config();

        let chain_config = match config.find_chain(&self.chain_id) {
            None => {
                return Output::error(format!(
                    "chain '{}' not found in configuration file",
                    self.chain_id
                ))
                .exit()
            }
            Some(chain_config) => chain_config,
        };

        debug!("Options: {:?}", self);

        let rt = Arc::new(TokioRuntime::new().unwrap());
        let chain = CosmosSdkChain::bootstrap(chain_config.clone(), rt).unwrap();

        let req = QueryChannelsRequest {
            pagination: ibc_proto::cosmos::base::query::pagination::all(),
        };

        let res = chain.query_channels(req);

        match res {
            Ok(channels) => {
                let ids: Vec<PortChannelId> = channels
                    .into_iter()
                    .map(|identified_channel| PortChannelId {
                        port_id: identified_channel.port_id,
                        channel_id: identified_channel.channel_id,
                    })
                    .collect();
                Output::success(ids).exit()
            }
            Err(e) => Output::error(format!("{}", e)).exit(),
        }
    }
}
