pub trait RewardInterface<T: frame_system::Config> {
	type Error;
	type Balance;

	fn reward_funds(
		rewarder_account_id: &T::AccountId,
		to_reward: &T::AccountId,
		reward: Self::Balance,
	) -> Result<(), Self::Error>;
	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error>;
}
