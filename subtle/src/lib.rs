use subtle::ConstantTimeEq;

static mut VALUE: [u64; 32] = [1; 32];

#[no_mangle]
pub extern "C" fn case_ct_eq_equal() {
  unsafe { VALUE = [1; 32]; }
}

#[no_mangle]
pub extern "C" fn case_ct_eq_inequal() {
  unsafe { VALUE = [u64::MAX; 32]; }
}

#[no_mangle]
pub extern "C" fn test_ct_eq() {
  core::hint::black_box((unsafe { VALUE }).ct_eq(&[1; 32]));
}
