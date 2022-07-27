use sp_std::{vec::Vec, convert::TryFrom};
use pallet_perun::{pallet::Config, types::{ParamsOf, StateOf, BalanceOf, ParticipantIndex}};

macro_rules! require {
  ($x:expr) => {
      if !$x {
        return false;
      }
  };
}

const NUM_PARTS: usize = 2;
const ACTOR_IDX: usize = 0;
const GRID_IDX: usize = 1;
const GRID_IDX_END: usize = 10;
const EMPTY: u8 = 0;
const PLAYER1: u8 = 1;
const PLAYER2: u8 = 2;

pub fn valid_transition<T: Config>(
  params: &ParamsOf<T>,
  from: &StateOf<T>,
  to: &StateOf<T>,
  signer: ParticipantIndex,
) -> bool {
  if params.participants.len() != NUM_PARTS {
    return false;
  } else if to.data.len() != GRID_IDX_END {
    return false;
  }

  // Check actor.
  let actor = from.data[ACTOR_IDX];
  if ACTOR_IDX != usize::try_from(signer).unwrap() {
    return false;
  } else if (actor + 1) % u8::try_from(NUM_PARTS).unwrap() != to.data[ACTOR_IDX] {
    return false;
  }

  // Check action.
  let mut changed = false;
  for i in GRID_IDX..GRID_IDX_END {
    // Check value.
    if !valid_value(to.data[i]) {
      return false;
    } else if from.data[i] != to.data[i] {
      if changed {
        return false;
      } else if from.data[i] != EMPTY {
        return false;
      }
      changed = true;
    }
  }

  // Check final.
  let (is_final,has_winner, winner) = check_final(&to.data);
  require!(to.finalized == is_final);

  // Check balances.
  let mut expected_balances = from.balances.clone();
  if has_winner {
      let loser: usize = 1 - winner;
      expected_balances[winner] = from.balances[winner] + from.balances[loser];
      expected_balances[loser] = BalanceOf::<T>::default();
  }
  require!(to.balances == expected_balances);
  return true;
}

fn valid_value(v: u8) -> bool {
  return v == EMPTY || v == PLAYER1 || v == PLAYER2;
}

fn check_final(data: &Vec<u8>) -> (bool, bool, usize) {
  // 0 1 2
  // 3 4 5
  // 6 7 8

  // Check winner.
  let winning: [[usize; 3]; 8] = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], // horizontal
    [0, 3, 6], [1, 4, 7], [2, 5, 8], // vertical
    [0, 4, 8], [2, 4, 6]             // diagonal
  ];
  for v in winning.iter() {
      let (ok, v) = same_value(data, v);
      if ok {
          if v == PLAYER1 {
              return (true, true, 0);
          } else if v == PLAYER2 {
              return (true, true, 1);
          }
      }
  }

  // Check all set.
  for f in data[GRID_IDX..GRID_IDX_END].iter() {
      if f != &EMPTY {
          return (false, false, 0);
      }
  }
  return (true, false, 0);
}

fn same_value(data: &Vec<u8>, v: &[usize; 3]) -> (bool, u8) {
  let first = data[GRID_IDX + v[0]];
  for f in v[1..].iter() {
      if data[GRID_IDX + f] != first {
          return (false, 0);
      }
  }
  return (true, first);
}