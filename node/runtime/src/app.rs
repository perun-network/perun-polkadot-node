use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use pallet_perun::{
	pallet::Config,
	types::{BalanceOf, ParamsOf, ParticipantIndex, StateOf},
};
use sp_std::{convert::TryFrom, vec::Vec};

macro_rules! require {
	($x:expr) => {
		if !$x {
			return false;
		}
	};
}

const NUM_PARTS: usize = 2;
const FIELD_EMPTY: u8 = 0;
const FIELD_PLAYER1: u8 = 1;
const FIELD_PLAYER2: u8 = 2;

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
struct TicTacToeAppData {
	pub next_actor: u8,
	pub grid: [u8; 9],
}

pub fn valid_transition<T: Config>(
	params: &ParamsOf<T>,
	from: &StateOf<T>,
	to: &StateOf<T>,
	signer: ParticipantIndex,
) -> bool {
	if params.participants.len() != NUM_PARTS {
		return false;
	}

	let from_data = TicTacToeAppData::decode(&mut from.data.as_slice()).unwrap();
	let to_data = TicTacToeAppData::decode(&mut to.data.as_slice()).unwrap();

	// Check actor.
	let actor = from_data.next_actor;
	let signer_u8 = u8::try_from(signer).unwrap();
	let num_parts = u8::try_from(NUM_PARTS).unwrap();
	if actor != signer_u8 {
		return false;
	} else if (actor + 1) % num_parts != to_data.next_actor {
		return false;
	}

	// Check action.
	let mut changed = false;
	for (i, v) in to_data.grid.iter().enumerate() {
		if !valid_value(*v) {
			return false; // Invalid value.
		}

		let from_v = from_data.grid[i];
		if from_v != *v {
			if changed {
				return false; // Two moves in one.
			} else if from_v != FIELD_EMPTY {
				return false; // Field overwritten.
			}
			changed = true;
		}
	}

	// Check final.
	let (is_final, has_winner, winner) = check_final(&to_data);
	require!(to.finalized == is_final);

	// Check balances.
	let actor_usize = usize::try_from(actor).unwrap();
	let expected_balances = compute_next_balances::<T>(&from.balances, actor_usize, has_winner, winner);
	require!(to.balances == expected_balances);
	return true;
}

fn compute_next_balances<T: Config>(balances: &[BalanceOf<T>], actor: usize, has_winner: bool, winner: usize) -> Vec::<BalanceOf::<T>> {
	let total = accumulate_balances::<T>(balances);
	let mut next_bals = Vec::<BalanceOf::<T>>::with_capacity(balances.len());
	for p in 0..next_bals.len() {
		if has_winner && winner == p || actor == p {
			next_bals[p] = total.clone();
		} else {
			next_bals[p] = BalanceOf::<T>::default();
		}
	}
	next_bals
}

fn accumulate_balances<T: Config>(balances: &[BalanceOf<T>]) -> BalanceOf<T> {
	let mut acc = BalanceOf::<T>::default();
	for b in balances.iter() {
		acc += *b;
	}
	return acc;
}

fn valid_value(v: u8) -> bool {
	return v == FIELD_EMPTY || v == FIELD_PLAYER1 || v == FIELD_PLAYER2;
}

fn check_final(data: &TicTacToeAppData) -> (bool, bool, usize) {
	// 0 1 2
	// 3 4 5
	// 6 7 8

	// Check winner.
	let winning: [[usize; 3]; 8] = [
		[0, 1, 2],
		[3, 4, 5],
		[6, 7, 8], // horizontal
		[0, 3, 6],
		[1, 4, 7],
		[2, 5, 8], // vertical
		[0, 4, 8],
		[2, 4, 6], // diagonal
	];
	for v in winning.iter() {
		let (ok, v) = same_value(data, v);
		if ok {
			if v == FIELD_PLAYER1 {
				return (true, true, 0);
			} else if v == FIELD_PLAYER2 {
				return (true, true, 1);
			}
		}
	}

	// Check all set.
	for f in data.grid.iter() {
		if f != &FIELD_EMPTY {
			return (false, false, 0);
		}
	}
	return (true, false, 0);
}

fn same_value(data: &TicTacToeAppData, v: &[usize; 3]) -> (bool, u8) {
	let first = data.grid[v[0]];
	for f in v[1..].iter() {
		if data.grid[*f] != first {
			return (false, 0);
		}
	}
	return (true, first);
}
