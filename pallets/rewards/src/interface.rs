pub trait RewardInterface<T: frame_system::Config> {
    type Error;
    type Balance;
    
    fn reward_funds(
        escrow_account_id: &T::AccountId,
        pallet_id: &T::AccountId,
        to_reward: &T::AccountId,
        reward: Self::Balance,
    ) -> Result<(), Self::Error>;
}
