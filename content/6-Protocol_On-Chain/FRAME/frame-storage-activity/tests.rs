use codec::Encode;
use frame_support::{assert_err, assert_noop, assert_ok};

use crate::{mock::Test as T, *};

fn give_balance_to_bob(who: &[u8]) -> Result<(), ()> {
	sp_io::storage::set(who, &100u32.encode());
	if who != b"bob" {
		// Too late to return an error, change is already made.
		return Err(())
	}

	Ok(())
}

#[test]
fn verify_first_write_last() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		assert_err!(give_balance_to_bob(b"alice"), ());
		let balance = sp_io::storage::get(b"alice").unwrap();
		assert_eq!(balance, 100u32.encode());
	});
}

#[test]
fn storage_value_key() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		use sp_core::hexdisplay::HexDisplay;
		println!("{}", HexDisplay::from(&Item1::<T>::hashed_key()));
		//assert!(false)
	});
}

#[test]
fn storage_value_exists() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// Nothing is actually there yet.
		assert_eq!(Item1::<T>::exists(), false);
		assert_eq!(Item1::<T>::try_get().ok(), None);
	});
}

#[test]
fn storage_value() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// Nothing is actually there yet.
		assert_eq!(Item1::<T>::exists(), false);
		assert_eq!(Item1::<T>::try_get().ok(), None);
	});
}

#[test]
fn value_query() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// `0u32` is the default value of `u32`
		assert_eq!(Item2::<T>::get(), 0u32);
		Item2::<T>::put(10u32);
		assert_eq!(Item2::<T>::get(), 10u32);
	});
}

#[test]
fn my_default() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// `42u32` is the configured `OnEmpty` value.
		assert_eq!(Item3::<T>::get(), 42u32);
		Item3::<T>::put(10u32);
		assert_eq!(Item3::<T>::get(), 10u32);
	});
}

#[test]
fn nonsense() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		assert_eq!(Item4::<T>::exists(), false);
		assert_eq!(Item5::<T>::exists(), false);
		Item4::<T>::put(None::<u32>);
		Item5::<T>::put(None::<u32>);
		assert_eq!(Item4::<T>::exists(), true);
		assert_eq!(Item5::<T>::exists(), true);
	});
}

#[test]
fn better_bool() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// false case
		assert_eq!(Item6::<T>::exists(), false);
		Item6::<T>::put(());
		// true case
		assert_eq!(Item6::<T>::exists(), true);
	});
}

#[test]
fn kill() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		assert_eq!(Item3::<T>::get(), 42u32);
		Item3::<T>::put(10u32);
		assert_eq!(Item3::<T>::get(), 10u32);
		//Item3::<T>::kill();
		let old_value = Item3::<T>::take();
		assert_eq!(Item3::<T>::get(), 42u32);
		assert_eq!(old_value, 10u32);
	});
}

#[test]
fn mutate() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		Item2::<T>::put(42u32);
		Item2::<T>::mutate(|x| {
			if *x % 2 == 0 {
				*x = *x / 2;
			}
		});
		assert_eq!(Item2::<T>::get(), 21);
	});
}

#[test]
fn try_mutate() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		Item2::<T>::put(42u32);
		assert_noop!(
			Item2::<T>::try_mutate(|x| -> Result<(), ()> {
				*x = *x / 2;
				if *x % 2 == 0 {
					Ok(())
				} else {
					Err(())
				}
			}),
			()
		);
		assert_eq!(Item2::<T>::get(), 42);
	});
}

#[test]
fn vec_tricks() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		assert_eq!(Item7::<T>::decode_len(), None);
		Item7::<T>::put(vec![0u8]);
		assert_eq!(Item7::<T>::decode_len(), Some(1));
		Item7::<T>::append(1u8);
		Item7::<T>::append(2u8);
		assert_eq!(Item7::<T>::get(), vec![0u8, 1u8, 2u8]);
		assert_eq!(Item7::<T>::decode_len(), Some(3));
	});
}

#[test]
fn bounded_vec() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		for i in 0u8..100u8 {
			assert_ok!(Item8::<T>::try_append(i));
		}
		assert_noop!(Item8::<T>::try_append(100), ());
	});
}

#[test]
fn storage_map() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		Item9::<T>::insert(0, 100);
		assert_eq!(Item9::<T>::get(0), Some(100));
		assert_eq!(Item9::<T>::get(1), None);
	});
}

#[test]
fn balance_map() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		// these would normally would be 32 byte addresses
		let alice = 0u64;
		let bob = 1u64;
		Item10::<T>::insert(alice, 100);

		let transfer = |from: u64, to: u64, amount: u128| -> Result<(), &'static str> {
			Item10::<T>::try_mutate(from, |from_balance| -> Result<(), &'static str> {
				Item10::<T>::try_mutate(to, |to_balance| -> Result<(), &'static str> {
					*to_balance = to_balance.checked_add(amount).ok_or("overflow")?;
					*from_balance = from_balance.checked_sub(amount).ok_or("not enough balance")?;
					Ok(())
				})
			})
		};

		assert_noop!(transfer(bob, alice, 10), "not enough balance");
		assert_ok!(transfer(alice, bob, 10));
		assert_noop!(transfer(alice, bob, 100), "not enough balance");

		assert_eq!(Item10::<T>::get(alice), 90);
		assert_eq!(Item10::<T>::get(bob), 10);
	});
}

#[test]
fn prefix_keys() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		use sp_core::hexdisplay::HexDisplay;
		println!("{}", HexDisplay::from(&Item2::<T>::hashed_key()));
		println!("{}", HexDisplay::from(&Item10::<T>::hashed_key_for(0)));
		println!("{}", HexDisplay::from(&Item10::<T>::hashed_key_for(1)));

		//assert!(false)
	});
}

#[test]
fn better_balance_map() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		for i in 0u64..10u64 {
			Item10::<T>::insert(i, u128::from(i * 100u64));
			Item11::<T>::insert(i, u128::from(i * 100u64));
		}
		// cannot call iter for 10 because it cannot returns the keys
		let all_10: Vec<_> = Item10::<T>::iter_values().collect();
		let all_11: Vec<_> = Item11::<T>::iter().collect();
		println!("{:?}\n{:?}", all_10, all_11);

		//assert!(false)
	});
}

#[test]
fn storage_n_map() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		Item12::<T>::insert((1u8, 1u16, 1u32), 1u128);
		assert_eq!(Item12::<T>::get((1u8, 1u16, 1u32)), Some(1u128));
	});
}
