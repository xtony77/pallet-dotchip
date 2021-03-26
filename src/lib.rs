#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::{Get, Currency, ReservableCurrency, WithdrawReasons, ExistenceRequirement}};
use frame_system::{ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
		pub Chip get(fn chip): map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
		pub MoneyPool get(fn money_pool): BalanceOf<T>;
	}
}

decl_event!(
	pub enum Event<T> where 
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
	{
		Deposit(AccountId, Balance),
		Withdraw(AccountId, Balance),
		Stake(AccountId, Balance),
		Draw(AccountId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		MoneyNotEnough,
		ChipNotEnough
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2)]
		pub fn deposit(origin, amount: BalanceOf<T>) {
			let player = ensure_signed(origin)?;
			let chip = Self::chip(&player);
			T::Currency::withdraw(&player, amount, WithdrawReasons::RESERVE, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough )?;
			<MoneyPool::<T>>::put(Self::money_pool() + amount);
			<Chip::<T>>::insert(&player, chip + amount);
			Self::deposit_event(RawEvent::Deposit(player, amount));
		}

		#[weight = 10_000 + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2)]
		pub fn withdraw(origin, amount: BalanceOf<T>) {
			let player = ensure_signed(origin)?;
			let chip = Self::chip(&player);
			ensure!(chip >= amount, Error::<T>::ChipNotEnough);
			T::Currency::deposit_creating(&player, amount);
			<MoneyPool::<T>>::put(Self::money_pool() - amount);
			<Chip::<T>>::insert(&player, chip - amount);
			Self::deposit_event(RawEvent::Withdraw(player, amount));
		}
	}
}

impl<T: Config> Module<T> {
	pub fn stake(player: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
		let chip = Self::chip(&player);
		ensure!(chip >= amount, Error::<T>::ChipNotEnough);
		<Chip::<T>>::insert(&player, chip - amount);
		Self::deposit_event(RawEvent::Stake(player, amount));
		Ok(())
	}

	pub fn draw(player: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
		let chip = Self::chip(&player);
		<Chip::<T>>::insert(&player, chip + amount);
		Self::deposit_event(RawEvent::Draw(player, amount));
		Ok(())
	}
}