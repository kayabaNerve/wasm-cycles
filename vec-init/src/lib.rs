#[no_mangle]
pub extern "C" fn case_vec_init1() {
  // do nothing
}

#[no_mangle]
pub extern "C" fn case_vec_init2() {
  // do nothing
}

#[no_mangle]
pub extern "C" fn case_vec_init3() {
  // do nothing
}

#[no_mangle]
pub extern "C" fn test_vec_init() {
  core::hint::black_box(vec![0; 1]);
}
