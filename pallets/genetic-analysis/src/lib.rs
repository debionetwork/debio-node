#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod interface;
pub mod weights;
pub use frame_support::{
	debug, dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime::traits::Hash,
	traits::Randomness,
};
pub use frame_system::pallet_prelude::*;
pub use interface::GeneticAnalysisInterface;
pub use primitives_tracking_id::{tracking_id_generator, TrackingId};
pub use sp_std::{fmt::Debug, prelude::*};
pub use traits_genetic_analysis::{GeneticAnalysisProvider, GeneticAnalysisTracking};
pub use traits_genetic_analysis_orders::{
	GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater,
};
pub use weights::WeightInfo;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum GeneticAnalysisStatus {
	Registered,
	InProgress,
	Rejected,
	ResultReady,
}
impl Default for GeneticAnalysisStatus {
	fn default() -> Self {
		Self::Registered
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalysis<AccountId, Hash, Moment> {
	pub genetic_analysis_tracking_id: TrackingId,
	pub genetic_analyst_id: AccountId,
	pub owner_id: AccountId,
	pub report_link: Vec<u8>,
	pub comment: Option<Vec<u8>>,
	pub rejected_title: Option<Vec<u8>>,
	pub rejected_description: Option<Vec<u8>>,
	pub genetic_analysis_order_id: Hash,
	pub created_at: Moment,
	pub updated_at: Moment,
	pub status: GeneticAnalysisStatus,
}
impl<AccountId, Hash, Moment: Copy> GeneticAnalysis<AccountId, Hash, Moment> {
	pub fn new(
		genetic_analyst_id: AccountId,
		genetic_analysis_order_id: Hash,
		genetic_analysis_tracking_id: TrackingId,
		owner_id: AccountId,
		created_at: Moment,
	) -> Self {
		Self {
			genetic_analyst_id,
			genetic_analysis_order_id,
			genetic_analysis_tracking_id,
			owner_id,
			report_link: Vec::<u8>::new(),
			comment: None,
			rejected_title: None,
			rejected_description: None,
			created_at,
			updated_at: created_at,
			status: GeneticAnalysisStatus::default(),
		}
	}
}
impl<AccountId, Hash, Moment> GeneticAnalysisTracking for GeneticAnalysis<AccountId, Hash, Moment> {
	fn get_genetic_analysis_tracking_id(&self) -> &TrackingId {
		&self.genetic_analysis_tracking_id
	}
	fn process_success(&self) -> bool {
		self.status == GeneticAnalysisStatus::ResultReady
	}
	fn is_rejected(&self) -> bool {
		self.status == GeneticAnalysisStatus::Rejected
	}
	fn is_empty(&self) -> bool {
		self.report_link == Vec::<u8>::new() && self.comment == None
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
		type GeneticAnalysisOrders: GeneticAnalysisOrderEventEmitter<Self>
			+ GeneticAnalysisOrderStatusUpdater<Self>;
		type GeneticAnalysisWeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// parameters [GeneticAnalysis]
		GeneticAnalysisSubmitted(GeneticAnalysisOf<T>),
		/// Received -> InProgress
		GeneticAnalysisInProgress(GeneticAnalysisOf<T>),
		/// QC Rejected
		GeneticAnalysisRejected(GeneticAnalysisOf<T>),
		/// ResultReady
		GeneticAnalysisResultReady(GeneticAnalysisOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		GeneticAnalysisOrderNotFound,
		GeneticAnalysisNotFound,
		GeneticAnalysisNotYetSubmitted,
		Unauthorized,
		TrackingIdCollision,
		ResultLinkRequired,
		ReportLinkRequired,
	}

	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type GeneticAnalysisOf<T> = GeneticAnalysis<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type DataHash<T> = <T as frame_system::Config>::Hash;

	// Storage ----------------
	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_by_genetic_analysis_tracking_id)]
	pub type GeneticAnalysisStorage<T> =
		StorageMap<_, Blake2_128Concat, TrackingId, GeneticAnalysisOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_by_owner_id)]
	pub type GeneticAnalysisByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<TrackingId>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_by_genetic_analyst_id)]
	pub type GeneticAnalysisByGeneticAnalyst<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<TrackingId>>;
	// --------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::GeneticAnalysisWeightInfo::reject_genetic_analysis())]
		pub fn reject_genetic_analysis(
			origin: OriginFor<T>,
			genetic_analysis_tracking_id: TrackingId,
			rejected_title: Vec<u8>,
			rejected_description: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisInterface<T>>::reject_genetic_analysis(
				&who,
				&genetic_analysis_tracking_id,
				&rejected_title,
				&rejected_description,
			) {
				Ok(genetic_analysis) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisRejected(genetic_analysis));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisWeightInfo::process_genetic_analysis())]
		pub fn process_genetic_analysis(
			origin: OriginFor<T>,
			genetic_analysis_tracking_id: TrackingId,
			status: GeneticAnalysisStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisInterface<T>>::process_genetic_analysis(
				&who,
				&genetic_analysis_tracking_id,
				status.clone(),
			) {
				Ok(genetic_analysis) => {
					if status == GeneticAnalysisStatus::ResultReady {
						Self::deposit_event(Event::<T>::GeneticAnalysisResultReady(
							genetic_analysis,
						))
					}

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisWeightInfo::submit_genetic_analysis())]
		pub fn submit_genetic_analysis(
			origin: OriginFor<T>,
			genetic_analysis_tracking_id: TrackingId,
			report_link: Vec<u8>,
			comment: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisInterface<T>>::submit_genetic_analysis(
				&who,
				&genetic_analysis_tracking_id,
				&report_link,
				&comment,
			) {
				Ok(genetic_analysis) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisSubmitted(genetic_analysis));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

impl<T: Config> GeneticAnalysisInterface<T> for Pallet<T> {
	type GeneticAnalysis = GeneticAnalysisOf<T>;
	type GeneticAnalysisStatus = GeneticAnalysisStatus;
	type Error = Error<T>;

	fn register_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		owner_id: &T::AccountId,
		genetic_analysis_order_id: &HashOf<T>,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		let seed = Self::generate_random_seed(genetic_analyst_id, owner_id);

		let mut tries = 10;
		loop {
			let tracking_id = tracking_id_generator::generate(seed.clone());
			let now = pallet_timestamp::Pallet::<T>::get();

			if !GeneticAnalysisStorage::<T>::contains_key(&tracking_id) {
				let genetic_analysis = GeneticAnalysis::new(
					genetic_analyst_id.clone(),
					*genetic_analysis_order_id,
					tracking_id,
					owner_id.clone(),
					now,
				);
				GeneticAnalysisStorage::<T>::insert(
					&genetic_analysis.genetic_analysis_tracking_id,
					&genetic_analysis,
				);
				Self::add_genetic_analysis_by_owner(&genetic_analysis);
				Self::add_genetic_analysis_by_genetic_analyst(&genetic_analysis);

				return Ok(genetic_analysis)
			}

			tries += 1;
			if tries > 10 {
				return Err(Error::<T>::TrackingIdCollision)
			}
		}
	}

	fn delete_genetic_analysis(
		tracking_id: &TrackingId,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		let genetic_analysis = GeneticAnalysisStorage::<T>::take(tracking_id);
		if genetic_analysis.is_none() {
			return Err(Error::<T>::GeneticAnalysisNotFound)
		}
		let genetic_analysis = genetic_analysis.unwrap();

		Ok(genetic_analysis)
	}

	fn reject_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		genetic_analysis_tracking_id: &TrackingId,
		rejected_title: &[u8],
		rejected_description: &[u8],
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		let genetic_analysis = GeneticAnalysisStorage::<T>::get(genetic_analysis_tracking_id);
		if genetic_analysis.is_none() {
			return Err(Error::<T>::GeneticAnalysisNotFound)
		}
		let mut genetic_analysis = genetic_analysis.unwrap();

		if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
			return Err(Error::<T>::Unauthorized)
		}

		let now = pallet_timestamp::Pallet::<T>::get();
		genetic_analysis.rejected_title = Some(rejected_title.to_vec());
		genetic_analysis.rejected_description = Some(rejected_description.to_vec());
		genetic_analysis.status = GeneticAnalysisStatus::Rejected;
		genetic_analysis.updated_at = now;

		GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);

		T::GeneticAnalysisOrders::emit_event_genetic_analysis_order_failed(
			&genetic_analysis.genetic_analysis_order_id,
		);
		T::GeneticAnalysisOrders::remove_genetic_analysis_order_id_from_pending_genetic_analysis_order_id_by_seller(
			&genetic_analysis.genetic_analyst_id,
			&genetic_analysis.genetic_analysis_order_id
		);
		T::GeneticAnalysisOrders::update_status_failed(&genetic_analysis.genetic_analysis_order_id);

		Ok(genetic_analysis)
	}

	fn process_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		genetic_analysis_tracking_id: &TrackingId,
		status: Self::GeneticAnalysisStatus,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		let genetic_analysis = GeneticAnalysisStorage::<T>::get(genetic_analysis_tracking_id);
		if genetic_analysis.is_none() {
			return Err(Error::<T>::GeneticAnalysisNotFound)
		}
		let mut genetic_analysis = genetic_analysis.unwrap();

		if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
			return Err(Error::<T>::Unauthorized)
		}

		if status == GeneticAnalysisStatus::ResultReady {
			let result = Self::genetic_analysis_by_genetic_analysis_tracking_id(
				genetic_analysis_tracking_id,
			)
			.unwrap();
			if result.is_empty() {
				return Err(Error::<T>::GeneticAnalysisNotYetSubmitted)
			}
			T::GeneticAnalysisOrders::remove_genetic_analysis_order_id_from_pending_genetic_analysis_order_id_by_seller(
				&genetic_analysis.genetic_analyst_id,
				&genetic_analysis.genetic_analysis_order_id
			);
		}

		let now = pallet_timestamp::Pallet::<T>::get();
		genetic_analysis.status = status;
		genetic_analysis.updated_at = now;
		GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);

		Ok(genetic_analysis)
	}

	fn submit_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		genetic_analysis_tracking_id: &TrackingId,
		report_link: &[u8],
		comment: &Option<Vec<u8>>,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		let genetic_analysis = GeneticAnalysisStorage::<T>::get(genetic_analysis_tracking_id);
		if genetic_analysis.is_none() {
			return Err(Error::<T>::GeneticAnalysisNotFound)
		}
		let mut genetic_analysis = genetic_analysis.unwrap();

		if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
			return Err(Error::<T>::Unauthorized)
		}

		let now = pallet_timestamp::Pallet::<T>::get();
		genetic_analysis.report_link = report_link.to_vec();
		genetic_analysis.comment = comment.clone();
		genetic_analysis.updated_at = now;

		GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);

		Ok(genetic_analysis)
	}

	fn genetic_analysis_by_genetic_analysis_tracking_id(
		genetic_analysis_tracking_id: &TrackingId,
	) -> Option<Self::GeneticAnalysis> {
		Self::genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id)
	}

	fn genetic_analysis_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<TrackingId>> {
		Self::genetic_analysis_by_owner_id(owner_id)
	}

	fn genetic_analysis_by_genetic_analyst_id(
		genetic_analyst_id: &T::AccountId,
	) -> Option<Vec<TrackingId>> {
		Self::genetic_analysis_by_genetic_analyst_id(genetic_analyst_id)
	}
}

impl<T: Config> GeneticAnalysisProvider<T> for Pallet<T> {
	type GeneticAnalysis = GeneticAnalysisOf<T>;
	type Error = Error<T>;

	fn register_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		owner_id: &T::AccountId,
		genetic_analysis_order_id: &HashOf<T>,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		<Self as GeneticAnalysisInterface<T>>::register_genetic_analysis(
			genetic_analyst_id,
			owner_id,
			genetic_analysis_order_id,
		)
	}

	fn delete_genetic_analysis(
		tracking_id: &TrackingId,
	) -> Result<Self::GeneticAnalysis, Self::Error> {
		<Self as GeneticAnalysisInterface<T>>::delete_genetic_analysis(tracking_id)
	}

	fn genetic_analysis_by_genetic_analysis_tracking_id(
		genetic_analysis_tracking_id: &TrackingId,
	) -> Option<Self::GeneticAnalysis> {
		<Self as GeneticAnalysisInterface<T>>::genetic_analysis_by_genetic_analysis_tracking_id(
			genetic_analysis_tracking_id,
		)
	}
}

impl<T: Config> Pallet<T> {
	pub fn generate_random_seed(creator_id: &T::AccountId, owner_id: &T::AccountId) -> Vec<u8> {
		let creator_info = frame_system::Pallet::<T>::account(creator_id);
		let creator_nonce = creator_info.nonce;
		let owner_info = frame_system::Pallet::<T>::account(owner_id);
		let owner_nonce = owner_info.nonce;

		let mut seed = creator_id.encode();
		seed.append(&mut creator_nonce.encode());
		seed.append(&mut owner_id.encode());
		seed.append(&mut owner_nonce.encode());

		T::RandomnessSource::random(&seed).encode()
	}

	pub fn add_genetic_analysis_by_owner(genetic_analysis: &GeneticAnalysisOf<T>) {
		match GeneticAnalysisByOwner::<T>::get(&genetic_analysis.owner_id) {
			None => {
				let genetic_analysis_tracking_ids =
					vec![genetic_analysis.genetic_analysis_tracking_id.clone()];
				GeneticAnalysisByOwner::<T>::insert(
					&genetic_analysis.owner_id,
					genetic_analysis_tracking_ids,
				);
			},
			Some(mut genetic_analysis_tracking_ids) => {
				genetic_analysis_tracking_ids
					.push(genetic_analysis.genetic_analysis_tracking_id.clone());
				GeneticAnalysisByOwner::<T>::insert(
					&genetic_analysis.owner_id,
					genetic_analysis_tracking_ids,
				);
			},
		}
	}

	pub fn add_genetic_analysis_by_genetic_analyst(genetic_analysis: &GeneticAnalysisOf<T>) {
		match GeneticAnalysisByGeneticAnalyst::<T>::get(&genetic_analysis.owner_id) {
			None => {
				let genetic_analysis_tracking_ids =
					vec![genetic_analysis.genetic_analysis_tracking_id.clone()];
				GeneticAnalysisByGeneticAnalyst::<T>::insert(
					&genetic_analysis.genetic_analyst_id,
					genetic_analysis_tracking_ids,
				);
			},
			Some(mut genetic_analysis_tracking_ids) => {
				genetic_analysis_tracking_ids
					.push(genetic_analysis.genetic_analysis_tracking_id.clone());
				GeneticAnalysisByGeneticAnalyst::<T>::insert(
					&genetic_analysis.genetic_analyst_id,
					genetic_analysis_tracking_ids,
				);
			},
		}
	}
}
