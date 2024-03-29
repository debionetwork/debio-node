use std::sync::Arc;

use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::{Block as BlockT, HashFor};

use sc_client_api::{backend::StateBackend, AuxStore, Backend, BlockBackend};
use sc_consensus_babe::{BabeApi, BabeConfiguration, Epoch};
use sc_consensus_babe_rpc::{Babe, BabeApiServer};
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};
use sc_rpc::{
	dev::{Dev, DevApiServer},
	SubscriptionTaskExecutor,
};
use sc_rpc_api::DenyUnsafe;
use sc_rpc_spec_v2::chain_spec::{ChainSpec, ChainSpecApiServer};
use sc_sync_state_rpc::{SyncState, SyncStateApiServer};
use sc_transaction_pool_api::TransactionPool;

use substrate_frame_rpc_system::{AccountNonceApi, System, SystemApiServer};

use beefy_gadget::notification::{BeefyBestBlockStream, BeefyVersionedFinalityProofStream};
use beefy_gadget_rpc::{Beefy, BeefyApiServer};
use pallet_mmr_rpc::{Mmr, MmrApiServer, MmrRuntimeApi};
use pallet_transaction_payment_rpc::{
	TransactionPayment, TransactionPaymentApiServer, TransactionPaymentRuntimeApi,
};

use debio_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index};

use jsonrpsee::RpcModule;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: BabeConfiguration,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA.
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_proof_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Dependencies for BEEFY.
pub struct BeefyDeps {
	/// Receives notifications about finality proof events from BEEFY.
	pub beefy_finality_proof_stream: BeefyVersionedFinalityProofStream<Block>,
	/// Receives notifications about best block events from BEEFY.
	pub beefy_best_block_stream: BeefyBestBlockStream<Block>,
	/// Executor to drive the subscription manager in the BEEFY RPC handler.
	pub beefy_subscription_executor: SubscriptionTaskExecutor,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// BEEFY specific dependencies.
	pub beefy: BeefyDeps,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Sync
		+ Send
		+ 'static,
	C::Api: BlockBuilder<Block>,
	C::Api: AccountNonceApi<Block, AccountId, Index>,
	C::Api: TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BabeApi<Block>,
	C::Api: MmrRuntimeApi<Block, <Block as BlockT>::Hash>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> + 'static,
	B: Backend<Block> + Send + Sync + 'static,
	B::State: StateBackend<HashFor<Block>>,
{
	let FullDeps { client, pool, select_chain, chain_spec, deny_unsafe, babe, grandpa, beefy } =
		deps;

	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;

	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_proof_provider,
	} = grandpa;

	let BeefyDeps {
		beefy_finality_proof_stream,
		beefy_best_block_stream,
		beefy_subscription_executor,
	} = beefy;

	let mut io = RpcModule::new(());
	let chain_name = chain_spec.name().to_string();
	let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
	let properties = chain_spec.properties();
	io.merge(ChainSpec::new(chain_name, genesis_hash, properties).into_rpc())?;
	io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	io.merge(
		Babe::new(
			client.clone(),
			shared_epoch_changes.clone(),
			keystore,
			babe_config,
			select_chain,
			deny_unsafe,
		)
		.into_rpc(),
	)?;
	io.merge(
		Grandpa::new(
			subscription_executor,
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			finality_proof_provider,
		)
		.into_rpc(),
	)?;
	io.merge(
		Beefy::<Block>::new(
			beefy_finality_proof_stream,
			beefy_best_block_stream,
			beefy_subscription_executor,
		)?
		.into_rpc(),
	)?;
	io.merge(Mmr::new(client.clone()).into_rpc())?;
	io.merge(
		SyncState::new(chain_spec, client.clone(), shared_authority_set, shared_epoch_changes)?
			.into_rpc(),
	)?;
	io.merge(Dev::new(client, deny_unsafe).into_rpc())?;

	Ok(io)
}
