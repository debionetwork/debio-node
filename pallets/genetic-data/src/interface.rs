use sp_std::vec::Vec;

pub trait GeneticDataInterface<T: frame_system::Config> {
    type Error;
    type GeneticDataId;
    type GeneticData;

    fn generate_genetic_data_id(
        owner_id: &T::AccountId,
        genetic_data_count: u64,
    ) -> Self::GeneticDataId;

    fn add_genetic_data(
        owner_id: &T::AccountId,
        title: &[u8],
        description: &[u8],
        report_link: &[u8],
    ) -> Result<Self::GeneticData, Self::Error>;

    fn update_genetic_data(
        owner_id: &T::AccountId,
        genetic_data_id: &T::Hash,
        title: &[u8],
        description: &[u8],
        report_link: &[u8],
    ) -> Result<Self::GeneticData, Self::Error>;

    fn remove_genetic_data(
        owner_id: &T::AccountId,
        genetic_data_id: &T::Hash,
    ) -> Result<Self::GeneticData, Self::Error>;

    fn genetic_data_count_by_owner(
        owner_id: &T::AccountId
    ) -> u64;

    fn genetic_data_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Vec<T::Hash>>;

    fn genetic_data_by_id(
        genetic_data_id: &Self::GeneticDataId,
    ) -> Option<Self::GeneticData>;
}
