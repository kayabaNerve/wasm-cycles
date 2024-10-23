use std::{io::Read, fs::File, collections::HashMap};

use wasmi::*;

fn main() {
  let mut file = File::open(std::env::args().skip(1).next().unwrap()).unwrap();
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).unwrap();

  let engine = Engine::new(&Config::default().consume_fuel(true));

  let module = Module::new(&engine, &buf).unwrap();
  let mut store = Store::new(module.engine(), ());
  let instance = Linker::<()>::new(module.engine())
    .instantiate(&mut store, &module)
    .unwrap()
    .ensure_no_start(&mut store)
    .unwrap();

  let mut tests = vec![];
  let mut cases = HashMap::new();
  for export in instance.exports(&store) {
    if let Some(test) = export.name().strip_prefix("test_") {
      tests.push(test.to_string());
      cases.insert(test.to_string(), vec![]);
    }
  }
  for export in instance.exports(&store) {
    if let Some(case) = export.name().strip_prefix("case_") {
      for test in &tests {
        if case.starts_with(test) {
          cases.get_mut(test).unwrap().push(export.name().to_string());
          break;
        }
      }
    }
  }

  for test in tests {
    let mut cycles = None;
    let cases = &cases[&test];

    for case in cases {
      store.set_fuel(u64::MAX).unwrap();
      instance
        .get_export(&store, case)
        .unwrap()
        .into_func()
        .unwrap()
        .call(&mut store, &[], &mut [])
        .unwrap();

      store.set_fuel(u64::MAX).unwrap();
      instance
        .get_export(&store, &("test_".to_string() + &test))
        .unwrap()
        .into_func()
        .unwrap()
        .call(&mut store, &[], &mut [])
        .unwrap();

      let this_cycles = u64::from(u64::MAX) - store.get_fuel().unwrap();
      if cycles.is_none() {
        cycles = Some(this_cycles);
      }
      assert_eq!(cycles.unwrap(), this_cycles, "{}", test);
    }
    println!("{test} passed ({} cases, {} cycles)", cases.len(), cycles.unwrap());
  }
}
