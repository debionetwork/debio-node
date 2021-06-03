#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub use pallet::*;

pub mod interface;
pub use interface::GeneticTestingInterface;
pub use frame_support::debug;
pub use frame_support::sp_runtime::traits::Hash;
pub use frame_support::pallet_prelude::*;
pub use frame_system::pallet_prelude::*;
pub use frame_support::dispatch::DispatchResultWithPostInfo;
pub use frame_support::traits::Randomness;
pub use sp_std::prelude::*;
pub use sp_std::fmt::Debug;
pub use traits_genetic_testing::{GeneticTestingProvider, DnaSampleTracking};
pub use traits_order::{OrderEventEmitter};

#[frame_support::pallet]
pub mod pallet {
    use crate::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RandomnessSource: Randomness<Self::Hash>;
        type Orders: OrderEventEmitter<Self>;
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
        /// parameters [DnaSample]
        DnaSampleCreated(DnaSampleOf<T>),
        /// Received
        DnaSampleReceived(DnaSampleOf<T>),
        /// Rejected
        DnaSampleRejected(DnaSampleOf<T>),
        /// Processing
        DnaSampleProcessing(DnaSampleOf<T>),
        /// Dna Sample Processed
        DnaSampleProcessed(DnaTestResultOf<T>),
        /// Dna Test Result Submitted
        DnaTestResultSubmitted(DnaTestResultOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        OrderNotFound,
        DnaSampleNotFound,
        Unauthorized,
        TrackingIdCollision,
        ResultLinkRequired,
    }

    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    pub type DnaSampleOf<T> = DnaSample<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
    pub type DnaTestResultOf<T> = DnaTestResult<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
    pub type DnaSampleTrackingId = Vec<u8>;


    // Storage ----------------
    #[pallet::storage]
    #[pallet::getter(fn dna_sample_by_tracking_id)]
    pub type DnaSamples<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, DnaSampleOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_test_result_by_tracking_id)]
    pub type DnaTestResults<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, DnaTestResultOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_samples_by_owner_id)]
    pub type DnaSamplesByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<DnaSampleTrackingId>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_samples_by_lab_id)]
    pub type DnaSamplesByLab<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<DnaSampleTrackingId>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_test_results_by_owner_id)]
    pub type DnaTestResultsByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<DnaSampleTrackingId>>;
    
    #[pallet::storage]
    #[pallet::getter(fn dna_test_results_by_lab_id)]
    pub type DnaTestResultsByLab<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<DnaSampleTrackingId>>;
    // --------------------------


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /*
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_dna_sample(origin: OriginFor<T>, dna_owner_id: T::AccountId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::create_dna_sample(&who, &dna_owner_id) {
                Ok(dna_sample) => {
                    Self::deposit_event(Event::<T>::DnaSampleCreated(dna_sample));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
        */

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn receive_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::receive_dna_sample(&who, &tracking_id) {
                Ok(dna_sample) => {
                    Self::deposit_event(Event::<T>::DnaSampleReceived(dna_sample.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn reject_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::reject_dna_sample(&who, &tracking_id) {
                Ok(dna_sample) => {
                    Self::deposit_event(Event::<T>::DnaSampleRejected(dna_sample.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn process_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::process_dna_sample(&who, &tracking_id) {
                Ok(dna_sample) => {
                    Self::deposit_event(Event::<T>::DnaSampleProcessing(dna_sample.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_test_result(origin: OriginFor<T>, tracking_id: Vec<u8>, is_success: bool, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::submit_test_result(&who, &tracking_id, is_success, &submission) {
                Ok(dna_test_result) => {
                    Self::deposit_event(Event::<T>::DnaSampleProcessed(dna_test_result.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_independent_test_result(origin: OriginFor<T>, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::submit_independent_test_result(&who, &submission) {
                Ok(dna_test_result) => {
                    Self::deposit_event(Event::<T>::DnaTestResultSubmitted(dna_test_result.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum DnaSampleStatus {
    Sending,
    Received,
    Rejected,
    Processing,
    Success,
    Failed,
}
impl Default for DnaSampleStatus {
    fn default() -> Self {
        Self::Sending
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaSample<AccountId, Hash, Moment> {
    tracking_id: Vec<u8>,
    lab_id: AccountId,
    owner_id: AccountId,
    status: DnaSampleStatus,
    order_id: Hash,
    created_at: Moment,
    updated_at: Moment,
}
impl<AccountId, Hash, Moment: Copy> DnaSample<AccountId, Hash, Moment> {
    pub fn new(tracking_id: Vec<u8>, lab_id: AccountId, owner_id: AccountId, order_id: Hash, created_at: Moment) -> Self {
        Self {
            tracking_id,
            lab_id,
            owner_id,
            status: DnaSampleStatus::default(),
            order_id,
            created_at: created_at,
            updated_at: created_at,
        }
    }
}
impl<AccountId, Hash, Moment> DnaSampleTracking for DnaSample<AccountId, Hash, Moment> {
    fn get_tracking_id(&self) -> &Vec<u8> {
        &self.tracking_id
    }
    fn process_success(&self) -> bool {
        self.status == DnaSampleStatus::Success
    }
    fn process_failed(&self) -> bool {
        self.status == DnaSampleStatus::Failed
    }
    fn is_rejected(&self) -> bool {
        self.status == DnaSampleStatus::Rejected
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaTestResult<AccountId, Hash, Moment> {
    tracking_id: Vec<u8>,
    lab_id: Option<AccountId>, // if lab_id == None, Test result is submitted independently
    owner_id: AccountId,
    comments: Option<Vec<u8>>,
    result_link: Option<Vec<u8>>,
    report_link: Option<Vec<u8>>,
    order_id: Option<Hash>,
    created_at: Moment,
    updated_at: Moment,
}
impl<AccountId, Hash, Moment: Copy> DnaTestResult<AccountId, Hash, Moment> {
    pub fn new(
        tracking_id: Vec<u8>,
        lab_id: Option<AccountId>,
        owner_id: AccountId,
        submission: DnaTestResultSubmission,
        order_id: Option<Hash>,
        created_at: Moment,
    )
        -> Self
    {
        Self {
            tracking_id,
            lab_id,
            owner_id,
            comments: submission.comments,
            result_link: submission.result_link,
            report_link: submission.report_link,
            order_id,
            created_at,
            updated_at: created_at,
        }
    }
}


#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaTestResultSubmission {
    comments: Option<Vec<u8>>,
    result_link: Option<Vec<u8>>,
    report_link: Option<Vec<u8>>,
}


impl<T: Config> GeneticTestingInterface<T> for Pallet<T> {
    type DnaSample = DnaSampleOf<T>;
    type DnaTestResult = DnaTestResultOf<T>;
    type DnaTestResultSubmission = DnaTestResultSubmission;
    type Error = Error<T>;

    fn create_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId, order_id: &HashOf<T>) -> Result<Self::DnaSample, Self::Error> {
        let seed = Self::generate_random_seed(lab_id, owner_id);

        let mut tries = 10;
        loop {
            let tracking_id = tracking_id_generator::generate(seed.clone());
            let now = pallet_timestamp::Pallet::<T>::get();

            if !DnaSamples::<T>::contains_key(&tracking_id) {
                let dna_sample = DnaSample::new(
                    tracking_id.clone(),
                    lab_id.clone(),
                    owner_id.clone(),
                    order_id.clone(),
                    now,
                );
                DnaSamples::<T>::insert(&dna_sample.tracking_id, &dna_sample);
                Self::add_dna_sample_by_owner(&dna_sample);
                Self::add_dna_sample_by_lab(&dna_sample);

                return Ok(dna_sample);
            }

            tries += 1;
            if tries > 10 {
                return Err(Error::<T>::TrackingIdCollision);
            }
        }

    }

    fn delete_dna_sample(tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error> {
        let dna_sample = DnaSamples::<T>::take(tracking_id);
        if dna_sample.is_none() {
            return Err(Error::<T>::DnaSampleNotFound);
        }
        let dna_sample = dna_sample.unwrap();

        Ok(dna_sample)
    }

    fn receive_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error> {
        let dna_sample = DnaSamples::<T>::get(tracking_id);
        if dna_sample.is_none() {
            return Err(Error::<T>::DnaSampleNotFound);
        }
        let mut dna_sample = dna_sample.unwrap();

        if dna_sample.lab_id != *lab_id {
            return Err(Error::<T>::Unauthorized)
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        dna_sample.status = DnaSampleStatus::Received;
        dna_sample.updated_at = now;

        DnaSamples::<T>::insert(tracking_id, &dna_sample);

        Ok(dna_sample)
    }

    fn reject_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error> {
        let dna_sample = DnaSamples::<T>::get(tracking_id);
        if dna_sample.is_none() {
            return Err(Error::<T>::DnaSampleNotFound);
        }
        let mut dna_sample = dna_sample.unwrap();

        if dna_sample.lab_id != *lab_id {
            return Err(Error::<T>::Unauthorized)
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        dna_sample.status = DnaSampleStatus::Rejected;
        dna_sample.updated_at = now;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);
        T::Orders::emit_event_order_failed(&dna_sample.order_id);

        Ok(dna_sample)
    }

    fn process_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error> {
        let dna_sample = DnaSamples::<T>::get(tracking_id);
        if dna_sample.is_none() {
            return Err(Error::<T>::DnaSampleNotFound);
        }
        let mut dna_sample = dna_sample.unwrap();

        if dna_sample.lab_id != *lab_id {
            return Err(Error::<T>::Unauthorized)
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        dna_sample.status = DnaSampleStatus::Processing;
        dna_sample.updated_at = now;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);

        Ok(dna_sample)
    }

    fn submit_test_result(
        lab_id: &T::AccountId,
        tracking_id: &Vec<u8>,
        is_success: bool,
        submission: &Self::DnaTestResultSubmission
    )
        -> Result<Self::DnaTestResult, Self::Error>
    {
        let dna_sample = DnaSamples::<T>::get(tracking_id);
        if dna_sample.is_none() {
            return Err(Error::<T>::DnaSampleNotFound);
        }
        let mut dna_sample = dna_sample.unwrap();

        if dna_sample.lab_id != *lab_id {
            return Err(Error::<T>::Unauthorized)
        }

        // Update DnaSample status
        match is_success {
            true => dna_sample.status = DnaSampleStatus::Success,
            false => dna_sample.status = DnaSampleStatus::Failed,
        }
        let now = pallet_timestamp::Pallet::<T>::get();
        dna_sample.updated_at = now;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);

        // Create DnaTestResult
        let dna_test_result = DnaTestResult::new(
            tracking_id.clone(),
            Some(lab_id.clone()), // Lab
            dna_sample.owner_id.clone(), // Owner
            submission.clone(),
            Some(dna_sample.order_id.clone()),
            now,
        );
        DnaTestResults::<T>::insert(tracking_id, &dna_test_result);
        Self::add_dna_test_results_by_lab(&dna_test_result);
        Self::add_dna_test_results_by_owner(&dna_test_result);
        
        Ok(dna_test_result)
    }

    fn submit_independent_test_result(
        owner_id: &T::AccountId,
        submission: &Self::DnaTestResultSubmission
    )
        -> Result<Self::DnaTestResult, Self::Error>
    {
        let seed = Self::generate_random_seed(owner_id, owner_id);

        if submission.result_link == None {
            return Err(Error::<T>::ResultLinkRequired);
        }

        let mut tries = 10;
        loop {
            let tracking_id = tracking_id_generator::generate(seed.clone());
            let now = pallet_timestamp::Pallet::<T>::get();

            if !DnaTestResults::<T>::contains_key(&tracking_id) {
                let dna_test_result = DnaTestResult::new(
                    tracking_id.clone(),
                    None, // Lab
                    owner_id.clone(),
                    submission.clone(),
                    None, // order_id
                    now,
                );
                DnaTestResults::<T>::insert(&tracking_id, &dna_test_result);
                Self::add_dna_test_results_by_owner(&dna_test_result);

                return Ok(dna_test_result);
            }

            tries += 1;
            if tries > 10 {
                return Err(Error::<T>::TrackingIdCollision);
            }
        }
    }

    fn dna_sample_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaSample> {
        Self::dna_sample_by_tracking_id(tracking_id)
    }

    fn dna_test_result_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaTestResult> {
        Self::dna_test_result_by_tracking_id(tracking_id)
    }

    // Return dna sample tracking ids
    fn dna_samples_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<Vec<u8>>> {
        Self::dna_samples_by_owner_id(owner_id)
    }

    // Return dna sample tracking ids
    fn dna_samples_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<Vec<u8>>> {
        Self::dna_samples_by_lab_id(lab_id)
    }

    // Return dna sample tracking ids
    fn dna_test_results_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<Vec<u8>>> {
        Self::dna_test_results_by_owner_id(owner_id)
    }

    // Return dna sample tracking ids
    fn dna_test_results_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<Vec<u8>>> {
        Self::dna_test_results_by_lab_id(lab_id)
    }
} 

impl<T: Config> GeneticTestingProvider<T> for Pallet<T> {
    type DnaSample = DnaSampleOf<T>;
    type Error = Error<T>;

    fn create_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId, order_id: &HashOf<T>) -> Result<Self::DnaSample, Self::Error> {
        <Self as GeneticTestingInterface<T>>::create_dna_sample(lab_id, owner_id, order_id)
    }
    fn dna_sample_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaSample> {
        <Self as GeneticTestingInterface<T>>::dna_sample_by_tracking_id(tracking_id)
    }

    fn delete_dna_sample(tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error> {
        <Self as GeneticTestingInterface<T>>::delete_dna_sample(tracking_id)
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

    pub fn add_dna_sample_by_owner(dna_sample: &DnaSampleOf<T>) {
        match DnaSamplesByOwner::<T>::get(&dna_sample.owner_id) {
            None => {
                let mut tracking_ids = Vec::<Vec<u8>>::new();
                tracking_ids.push(dna_sample.tracking_id.clone());
                DnaSamplesByOwner::<T>::insert(&dna_sample.owner_id, tracking_ids);
            },
            Some(mut tracking_ids) => {
                tracking_ids.push(dna_sample.tracking_id.clone());
                DnaSamplesByOwner::<T>::insert(&dna_sample.owner_id, tracking_ids);
            }
        }
    }

    pub fn add_dna_sample_by_lab(dna_sample: &DnaSampleOf<T>) {
        match DnaSamplesByLab::<T>::get(&dna_sample.owner_id) {
            None => {
                let mut tracking_ids = Vec::<Vec<u8>>::new();
                tracking_ids.push(dna_sample.tracking_id.clone());
                DnaSamplesByLab::<T>::insert(&dna_sample.lab_id, tracking_ids);
            },
            Some(mut tracking_ids) => {
                tracking_ids.push(dna_sample.tracking_id.clone());
                DnaSamplesByLab::<T>::insert(&dna_sample.lab_id, tracking_ids);
            }
        }
    }

    pub fn add_dna_test_results_by_owner(dna_test_result: &DnaTestResultOf<T>) {
        match DnaTestResultsByOwner::<T>::get(&dna_test_result.owner_id) {
            None => {
                let mut tracking_ids = Vec::<Vec<u8>>::new();
                tracking_ids.push(dna_test_result.tracking_id.clone());
                DnaTestResultsByOwner::<T>::insert(&dna_test_result.owner_id, tracking_ids);
            },
            Some(mut tracking_ids) => {
                tracking_ids.push(dna_test_result.tracking_id.clone());
                DnaTestResultsByOwner::<T>::insert(&dna_test_result.owner_id, tracking_ids);
            }
        }
    }

    pub fn add_dna_test_results_by_lab(dna_test_result: &DnaTestResultOf<T>) {
        match DnaTestResultsByLab::<T>::get(&dna_test_result.owner_id) {
            None => {
                let mut tracking_ids = Vec::<Vec<u8>>::new();
                tracking_ids.push(dna_test_result.tracking_id.clone());
                DnaTestResultsByLab::<T>::insert(&dna_test_result.owner_id, tracking_ids);
            },
            Some(mut tracking_ids) => {
                tracking_ids.push(dna_test_result.tracking_id.clone());
                DnaTestResultsByLab::<T>::insert(&dna_test_result.owner_id, tracking_ids);
            }
        }
    }
}


/// Human Readable Tracking ID
pub mod tracking_id_generator {
    use crate::*;

    pub const SAFE: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        // 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        // 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn generate(seed: Vec<u8>) -> Vec<u8> {
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
                        return id.iter().map(|c| *c as u8).collect::<Vec<_>>();
                    }
                }
            }
        }
    }
}

