pub trait RewardInterface<T: frame_system::Config> {
    type Error;
    type Balance;
    
    fn reward_funds(
        rewarder_account_id: &T::AccountId,
        to_reward: &T::AccountId,
        reward: Self::Balance,
    ) -> Result<(), Self::Error>;
    
    fn add_total_reward_balance(
        sudo_account_id: &T::AccountId,
        reward: Self::Balance,
    ) -> Result<(), Self::Error>;
}
