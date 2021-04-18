#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

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

#[frame_support::pallet]
pub mod pallet {
    use crate::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RandomnessSource: Randomness<Self::Hash>;
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
        DnaSampleCreated(DnaSample<T::AccountId>),
        /// Received
        DnaSampleReceived(DnaSample<T::AccountId>),
        /// Rejected
        DnaSampleRejected(DnaSample<T::AccountId>),
        /// Processing
        DnaSampleProcessing(DnaSample<T::AccountId>),
        /// Dna Sample Processed
        DnaSampleProcessed(DnaTestResultOf<T>)
    }

    #[pallet::error]
    pub enum Error<T> {
        OrderNotFound,
        DnaSampleNotFound,
        Unauthorized,
        TrackingIdCollision,
    }

    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type DnaSampleOf<T> = DnaSample<AccountIdOf<T>>;
    pub type DnaTestResultOf<T> = DnaTestResult<HashOf<T>, AccountIdOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_sample_by_tracking_id)]
    pub type DnaSamples<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, DnaSampleOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn dna_test_result_by_tracking_id)]
    pub type DnaTestResults<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, DnaTestResultOf<T>>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
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
        pub fn submit_test_result(origin: OriginFor<T>, tracking_id: Vec<u8>, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticTestingInterface<T>>::submit_test_result(&who, &tracking_id, &submission) {
                Ok(dna_test_result) => {
                    Self::deposit_event(Event::<T>::DnaSampleProcessed(dna_test_result.clone()));
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
    Processed,
}
impl Default for DnaSampleStatus {
    fn default() -> Self {
        Self::Sending
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaSample<AccountId> {
    tracking_id: Vec<u8>,
    lab_id: AccountId,
    owner_id: AccountId,
    status: DnaSampleStatus,
}
impl<AccountId> DnaSample<AccountId> {
    pub fn new(tracking_id: Vec<u8>, lab_id: AccountId, owner_id: AccountId) -> Self {
        Self {
            tracking_id,
            lab_id,
            owner_id,
            status: DnaSampleStatus::default(),
        }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum DnaTestResultStatus {
    Success,
    Failed,
}
impl Default for DnaTestResultStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaTestResult<Hash, AccountId> {
    dna_sample_tracking_id: Vec<u8>,
    lab_id: AccountId,
    owner_id: AccountId,
    status: DnaTestResultStatus,
    comments: Option<Vec<u8>>,
    result_link: Option<Vec<u8>>,
    report_link: Option<Vec<u8>>,
    genetic_token_id: Option<Hash>,
}
impl<Hash, AccountId> DnaTestResult<Hash, AccountId> {
    pub fn new(
        dna_sample_tracking_id: Vec<u8>,
        lab_id: AccountId,
        owner_id: AccountId,
        submission: DnaTestResultSubmission,
        genetic_token_id: Option<Hash>,
    )
        -> Self
    {
        Self {
            dna_sample_tracking_id,
            lab_id,
            owner_id,
            status: submission.status,
            comments: submission.comments,
            result_link: submission.result_link,
            report_link: submission.report_link,
            genetic_token_id
        }
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct DnaTestResultSubmission {
    status: DnaTestResultStatus,
    comments: Option<Vec<u8>>,
    result_link: Option<Vec<u8>>,
    report_link: Option<Vec<u8>>,
}


impl<T: Config> GeneticTestingInterface<T> for Pallet<T> {
    type DnaSample = DnaSampleOf<T>;
    type DnaTestResult = DnaTestResult<T::Hash, T::AccountId>;
    type DnaTestResultSubmission = DnaTestResultSubmission;
    type Error = Error<T>;

    fn create_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId) -> Result<Self::DnaSample, Self::Error> {
        let seed = Self::generate_random_seed(lab_id, owner_id);

        let mut tries = 10;
        loop {
            let tracking_id = tracking_id_generator::generate(seed.clone());

            if !DnaSamples::<T>::contains_key(&tracking_id) {
                let dna_sample = DnaSample::new(tracking_id.clone(), lab_id.clone(), owner_id.clone());
                DnaSamples::<T>::insert(&dna_sample.tracking_id, &dna_sample);

                return Ok(dna_sample);
            }

            tries += 1;
            if tries > 10 {
                return Err(Error::<T>::TrackingIdCollision);
            }
        }

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

        dna_sample.status = DnaSampleStatus::Received;
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

        dna_sample.status = DnaSampleStatus::Rejected;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);

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

        dna_sample.status = DnaSampleStatus::Processing;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);

        Ok(dna_sample)
    }

    fn submit_test_result(
        lab_id: &T::AccountId,
        tracking_id: &Vec<u8>,
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
        dna_sample.status = DnaSampleStatus::Processed;
        DnaSamples::<T>::insert(tracking_id, &dna_sample);

        if submission.status == DnaTestResultStatus::Failed {
            // Create DnaTestResult
            let dna_test_result = DnaTestResult::new(
                tracking_id.clone(),
                lab_id.clone(), // Lab
                dna_sample.owner_id.clone(), // Owner
                submission.clone(),
                None, // genetic_token_id
            );
            DnaTestResults::<T>::insert(tracking_id, &dna_test_result);
            
            return Ok(dna_test_result);
        }
        // TODO: let genetic_token_id = Mint GeneticMaterialNFT ()
        let genetic_token_id = None;

        // Create DnaTestResult
        let dna_test_result = DnaTestResult::new(
            tracking_id.clone(),
            lab_id.clone(), // Lab
            dna_sample.owner_id.clone(), // Owner
            submission.clone(),
            genetic_token_id, // genetic_token_id
        );
        DnaTestResults::<T>::insert(tracking_id, &dna_test_result);
        
        return Ok(dna_test_result);
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
}


pub mod tracking_id_generator {
    use crate::*;

    pub const SAFE: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
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

