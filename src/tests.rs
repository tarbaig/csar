use super::FDVar;
use super::Domain;

#[test]
fn creates_new_var() {
   let x = FDVar::new(-2, 255, ~"x");
   assert!(x.min() == -2);
   assert!(x.max() == 255);
}

fn min_is_min(d: &Domain) -> bool {
   match d.intervals[0] {
      (x, _) => x == d.min
   }
}

fn max_is_max(d: &Domain) -> bool {
   match d.intervals[d.intervals.len() - 1] {
      (_, y) => y == d.max
   }
}

fn setup_domain_simple() -> Domain {
   Domain { min: -3, max: 64, intervals: ~[(-3, 2), (4, 42), (54, 64)] }
}

fn teardown(d: &Domain) {
   assert!(min_is_min(d));
   assert!(max_is_max(d));
}

#[test]
fn sets_min_lower() {
   let mut d = setup_domain_simple();
   d.set_min(-4);
   assert!(d.min == -3);
   teardown(&d);
}

#[test]
fn sets_min_middle() {
   let mut d = setup_domain_simple();
   let values = ~[-2, 8, 42, 54, 64];
   for &i in values.iter() {
      d.set_min(i);
      assert!(d.min == i);
   }
   teardown(&d);
}

#[test]
fn sets_min_in_hole() {
   let mut d = setup_domain_simple();
   d.set_min(43);
   assert!(d.min == 54);
   teardown(&d);
}

#[test]
// #[should_fail]
fn sets_min_too_high() {
   let mut d = setup_domain_simple();
   d.set_min(65);
   assert!(d.min == -3);
   teardown(&d);
}

#[test]
fn sets_max_higher() {
   let mut d = setup_domain_simple();
   d.set_max(65);
   assert!(d.max == 64);
   teardown(&d);
}

#[test]
fn sets_max_middle() {
   let mut d = setup_domain_simple();
   let values = ~[63, 54, 42, 8, -3];
   for &i in values.iter() {
      d.set_max(i);
      assert!(d.max == i);
   }
   teardown(&d);
}

#[test]
fn sets_max_in_hole() {
   let mut d = setup_domain_simple();
   d.set_max(43);
   assert!(d.max == 42);
   teardown(&d);
}

#[test]
// #[should_fail]
fn sets_max_too_low() {
   let mut d = setup_domain_simple();
   d.set_max(-4);
   assert!(d.max == 64);
   teardown(&d);
}

fn setup_domain_holy() -> Domain {
   Domain { min: -3, max: 64, intervals: ~[(-3, 2), (4, 18), (20, 24),
      (30, 30), (32, 34), (36, 38), (40, 42), (54, 64)] }
}

#[test]
fn remove_outside() {
   let mut d = setup_domain_holy();
   let e = setup_domain_holy();
   d.remove(-8);
   d.remove(3);
   d.remove(19);
   d.remove(31);
   d.remove(35);
   d.remove(48);
   d.remove(128);
   assert!(d.intervals.len() == e.intervals.len());
   for i in range(0, d.intervals.len()) {
      assert!(d.intervals[i] == e.intervals[i]);
   }
}
