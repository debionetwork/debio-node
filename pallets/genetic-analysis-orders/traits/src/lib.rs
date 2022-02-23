#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait GeneticAnalysisOrderEventEmitter<T: Config> {
	fn emit_event_genetic_analysis_order_failed(genetic_analysis_order_id: &T::Hash);
}

pub trait GeneticAnalysisOrderStatusUpdater<T: Config> {
	fn update_status_failed(genetic_analysis_order_id: &T::Hash);
	fn remove_genetic_analysis_order_id_from_pending_genetic_analysis_order_id_by_seller(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash
	);
}
