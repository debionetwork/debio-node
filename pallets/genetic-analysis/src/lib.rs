#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use scale_info::TypeInfo;

pub mod weights;
pub mod interface;
pub use weights::WeightInfo;
pub use frame_support::debug;
pub use frame_support::dispatch::DispatchResultWithPostInfo;
pub use frame_support::pallet_prelude::*;
pub use frame_support::sp_runtime::traits::Hash;
pub use frame_support::traits::Randomness;
pub use frame_system::pallet_prelude::*;
pub use interface::GeneticAnalysisInterface;
pub use sp_std::fmt::Debug;
pub use sp_std::prelude::*;
pub use primitives_tracking_id::TrackingId;
pub use traits_genetic_analysis::{GeneticAnalysisTracking, GeneticAnalysisProvider};
pub use traits_genetic_analysis_orders::{GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum GeneticAnalysisStatus {
    Registered,
    InProgress,
    Rejected,
    ResultReady
}
impl Default for GeneticAnalysisStatus {
    fn default() -> Self {
        Self::Registered
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalysis<AccountId, Hash, Moment> {
    genetic_analysis_tracking_id: TrackingId,
    genetic_analyst_id: AccountId,
    owner_id: AccountId,
    comment: Vec<u8>,
    report_link: Vec<u8>,
    genetic_analysis_order_id: Hash,
    created_at: Moment,
    updated_at: Moment,
    status: GeneticAnalysisStatus,
}
impl<AccountId, Hash, Moment: Copy> GeneticAnalysis<AccountId, Hash, Moment> {
    pub fn new(
        genetic_analysis_tracking_id: TrackingId,
        genetic_analyst_id: AccountId,
        owner_id: AccountId,
        comment: Vec<u8>,
        report_link: Vec<u8>,
        genetic_analysis_order_id: Hash,
        created_at: Moment,
    ) -> Self {
        Self {
            genetic_analysis_tracking_id,
            genetic_analyst_id,
            owner_id,
            comment,
            report_link,
            genetic_analysis_order_id,
            created_at: created_at,
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
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalysisSubmission {
    pub comments: Vec<u8>,
    pub report_link: Vec<u8>,
}

#[frame_support::pallet]
pub mod pallet {
    use crate::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
        type GeneticAnalysisOrders: GeneticAnalysisOrderEventEmitter<Self> + GeneticAnalysisOrderStatusUpdater<Self>;
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
        GeneticAnalysisRegistered(GeneticAnalysisOf<T>),
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
    pub type GeneticAnalysisStorage<T> = StorageMap<_, Blake2_128Concat, TrackingId, GeneticAnalysisOf<T>>;

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
                    Self::deposit_event(Event::<T>::GeneticAnalysisRejected(genetic_analysis.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
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
                    match status {
                        GeneticAnalysisStatus::QualityControlled => Self::deposit_event(
                            Event::<T>::GeneticAnalysisQualityControlled(genetic_analysis.clone()),
                        ),
                        GeneticAnalysisStatus::ResultReady => Self::deposit_event(
                            Event::<T>::GeneticAnalysisResultReady(genetic_analysis.clone()),
                        ),
                        _ => (),
                    }

                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(T::GeneticAnalysisWeightInfo::submit_genetic_analysis())]
        pub fn submit_genetic_analysis(
            origin: OriginFor<T>,
            genetic_analysis_tracking_id: TrackingId,
            submission: GeneticAnalysisSubmission,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticAnalysisInterface<T>>::submit_genetic_analysis(
                &who,
                &genetic_analysis_tracking_id,
                &submission,
            ) {
                Ok(genetic_analysis) => {
                    Self::deposit_event(Event::<T>::GeneticAnalysisSubmitted(
                        genetic_analysis.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
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
            let genetic_analysis_tracking_id = genetic_analysis_tracking_id_generator::generate(seed.clone());
            let now = pallet_timestamp::Pallet::<T>::get();

            if !GeneticAnalysisStorage::<T>::contains_key(&genetic_analysis_tracking_id) {
                let genetic_analysis = GeneticAnalysis::new(
                    genetic_analysis_tracking_id.clone(),
                    genetic_analyst_id.clone(),
                    owner_id.clone(),
                    genetic_analysis_order_id.clone(),
                    now,
                );
                GeneticAnalysisStorage::<T>::insert(&genetic_analysis.genetic_analysis_tracking_id, &genetic_analysis);
                Self::add_genetic_analysis_by_owner(&genetic_analysis);
                Self::add_genetic_analysis_by_genetic_analyst(&genetic_analysis);

                return Ok(genetic_analysis);
            }

            tries += 1;
            if tries > 10 {
                return Err(Error::<T>::TrackingIdCollision);
            }
        }
    }

    fn delete_genetic_analysis(
        genetic_analysis_tracking_id: &TrackingId
    ) -> Result<Self::GeneticAnalysis, Self::Error> {
        let genetic_analysis = GeneticAnalysisStorage::<T>::take(genetic_analysis_tracking_id);
        if genetic_analysis.is_none() {
            return Err(Error::<T>::GeneticAnalysisNotFound);
        }
        let genetic_analysis = genetic_analysis.unwrap();

        Ok(genetic_analysis)
    }

    fn reject_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        genetic_analysis_tracking_id: &TrackingId,
        rejected_title: &Vec<u8>,
        rejected_description: &Vec<u8>,
    ) -> Result<Self::GeneticAnalysis, Self::Error> {
        let genetic_analysis = GeneticAnalysisStorage::<T>::get(genetic_analysis_tracking_id);
        if genetic_analysis.is_none() {
            return Err(Error::<T>::GeneticAnalysisNotFound);
        }
        let mut genetic_analysis = genetic_analysis.unwrap();

        if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
            return Err(Error::<T>::Unauthorized);
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        genetic_analysis.rejected_title = Some(rejected_title.clone());
        genetic_analysis.rejected_description = Some(rejected_description.clone());
        genetic_analysis.status = GeneticAnalysisStatus::Rejected;
        genetic_analysis.updated_at = now;
        GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);
        T::GeneticAnalysisOrders::emit_event_order_failed(&genetic_analysis.genetic_analysis_order_id);
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
            return Err(Error::<T>::GeneticAnalysisNotFound);
        }
        let mut genetic_analysis = genetic_analysis.unwrap();

        if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
            return Err(Error::<T>::Unauthorized);
        }

        if status == GeneticAnalysisStatus::ResultReady {
            let result = Self::genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id);
            if result.is_none() {
                return Err(Error::<T>::GeneticAnalysisNotYetSubmitted);
            }
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        genetic_analysis.status = status.clone();
        genetic_analysis.updated_at = now;
        GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);

        Ok(genetic_analysis)
    }

    fn submit_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        genetic_analysis_tracking_id: &TrackingId,
        submission: &Self::GeneticAnalysisSubmission,
    ) -> Result<Self::GeneticAnalysis, Self::Error> {
        let genetic_analysis = GeneticAnalysisStorage::<T>::get(genetic_analysis_tracking_id);
        if genetic_analysis.is_none() {
            return Err(Error::<T>::GeneticAnalysisNotFound);
        }
        let mut genetic_analysis = genetic_analysis.unwrap();

        if genetic_analysis.genetic_analyst_id != *genetic_analyst_id {
            return Err(Error::<T>::Unauthorized);
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        genetic_analysis.updated_at = now;
        GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);

        // Create GeneticAnalysis
        let genetic_analysis = GeneticAnalysis::new(
            genetic_analysis_tracking_id.clone(),
            Some(genetic_analyst_id.clone()),        // GeneticAnalyst
            genetic_analysis.owner_id.clone(), // Owner
            submission.clone(),
            Some(genetic_analysis.genetic_analysis_order_id.clone()),
            now,
        );
        GeneticAnalysisStorage::<T>::insert(genetic_analysis_tracking_id, &genetic_analysis);
        Self::add_genetic_analysis_by_genetic_analyst(&genetic_analysis);
        Self::add_genetic_analysis_by_owner(&genetic_analysis);

        Ok(genetic_analysis)
    }

    fn genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id: &TrackingId) -> Option<Self::GeneticAnalysis> {
        Self::genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id)
    }

    fn genetic_analysis_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<TrackingId>> {
        Self::genetic_analysis_by_owner_id(owner_id)
    }

    fn genetic_analysis_by_genetic_analyst_id(genetic_analyst_id: &T::AccountId) -> Option<Vec<TrackingId>> {
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
        <Self as GeneticAnalysisInterface<T>>::register_genetic_analysis(genetic_analyst_id, owner_id, genetic_analysis_order_id)
    }
    fn genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id: &TrackingId) -> Option<Self::GeneticAnalysis> {
        <Self as GeneticAnalysisInterface<T>>::genetic_analysis_by_genetic_analysis_tracking_id(genetic_analysis_tracking_id)
    }

    fn delete_genetic_analysis(genetic_analysis_tracking_id: &TrackingId) -> Result<Self::GeneticAnalysis, Self::Error> {
        <Self as GeneticAnalysisInterface<T>>::delete_genetic_analysis(genetic_analysis_tracking_id)
    }
}

impl<T: Config> Pallet<T> {
    pub fn generate_random_seed(creator_id: &T::AccountId, owner_id: &T::AccountId) -> Vec<u8> {
        let creator_info = frame_system::Pallet::<T>::account(creator_id);
        let creator_nonce = creator_info.nonce.clone();
        let owner_info = frame_system::Pallet::<T>::account(owner_id);
        let owner_nonce = owner_info.nonce.clone();

        let mut seed = creator_id.encode();
        seed.append(&mut creator_nonce.encode());
        seed.append(&mut owner_id.encode());
        seed.append(&mut owner_nonce.encode());

        T::RandomnessSource::random(&seed).encode()
    }

    pub fn add_genetic_analysis_by_owner(genetic_analysis: &GeneticAnalysisOf<T>) {
        match GeneticAnalysisByOwner::<T>::get(&genetic_analysis.owner_id) {
            None => {
                let mut genetic_analysis_tracking_ids = Vec::<TrackingId>::new();
                genetic_analysis_tracking_ids.push(genetic_analysis.genetic_analysis_tracking_id.clone());
                GeneticAnalysisByOwner::<T>::insert(&genetic_analysis.owner_id, genetic_analysis_tracking_ids);
            }
            Some(mut genetic_analysis_tracking_ids) => {
                genetic_analysis_tracking_ids.push(genetic_analysis.genetic_analysis_tracking_id.clone());
                GeneticAnalysisByOwner::<T>::insert(&genetic_analysis.owner_id, genetic_analysis_tracking_ids);
            }
        }
    }

    pub fn add_genetic_analysis_by_genetic_analyst(genetic_analysis: &GeneticAnalysisOf<T>) {
        match GeneticAnalysisByGeneticAnalyst::<T>::get(&genetic_analysis.owner_id) {
            None => {
                let mut genetic_analysis_tracking_ids = Vec::<TrackingId>::new();
                genetic_analysis_tracking_ids.push(genetic_analysis.genetic_analysis_tracking_id.clone());
                GeneticAnalysisByGeneticAnalyst::<T>::insert(&genetic_analysis.genetic_analyst_id, genetic_analysis_tracking_ids);
            }
            Some(mut genetic_analysis_tracking_ids) => {
                genetic_analysis_tracking_ids.push(genetic_analysis.genetic_analysis_tracking_id.clone());
                GeneticAnalysisByGeneticAnalyst::<T>::insert(&genetic_analysis.genetic_analyst_id, genetic_analysis_tracking_ids);
            }
        }
    }
}

/// Human Readable Tracking ID
pub mod genetic_analysis_tracking_id_generator {
    use crate::*;

    pub const SAFE: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        // 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        // 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn generate(seed: Vec<u8>) -> TrackingId {
        let alphabet = &SAFE;
        let size = 21;
        let mask = alphabet.len().next_power_of_two() - 1;

        // Assert that the masking does not truncate the alphabet. (See #9)
        debug_assert!(alphabet.len() <= mask + 1);

        let mut id = Vec::new();

        loop {
            for &byte in &seed {
                let byte = byte as usize & mask;

                if alphabet.len() > byte {
                    id.push(alphabet[byte]);

                    if id.len() == size {
                        let _vec_id = id.iter()
                            .map(|c| *c as u8)
                            .collect::<Vec<_>>();
                        
                        let _dna_genetic_analysis_tracking_id = TrackingId::from_vec(_vec_id);
                        return _dna_genetic_analysis_tracking_id;
                    }
                }
            }
        }
    }
}
