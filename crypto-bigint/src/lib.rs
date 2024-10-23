use rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crypto_bigint::*;

static mut LHS: U256 = U256::ZERO;
static mut RHS: U256 = U256::ZERO;

#[no_mangle]
pub extern "C" fn case_add_zero() {
unsafe {
  LHS = U256::ZERO;
  RHS = U256::ZERO;
}
}

#[no_mangle]
pub extern "C" fn case_add_rand() {
  let mut rng = ChaCha20Rng::from_seed([0xff; 32]);
  unsafe {
    LHS = U256::random(&mut rng);
    RHS = U256::random(&mut rng);
  }
}

#[no_mangle]
pub extern "C" fn test_add() {
  unsafe {
    core::hint::black_box(LHS.wrapping_add(&RHS));
    core::hint::black_box(LHS.checked_add(&RHS));
  }
}
