pub trait RewardInterface<T: frame_system::Config> {
    type Error;
    type Balance;

    fn slash_funds(
        escrow_account_id: &T::AccountId,
        to_punish: &T::AccountId,
        collateral: Self::Balance,
    ) -> Result<(), Self::Error>;
    fn reward_funds(
        escrow_account_id: &T::AccountId,
        to_reward: &T::AccountId,
        reward: Self::Balance,
    ) -> Result<(), Self::Error>;
}
