#![feature(const_trait_bound_opt_out)]
#![feature(associated_type_bounds)]
#![allow(incomplete_features)]

trait T {}
struct S;
impl T for S {}

fn rpit() -> impl ?const T { S }
//~^ ERROR `?const` is not permitted in `impl Trait`
//~| ERROR `?const` on trait bounds is not yet implemented

fn apit(_: impl ?const T) {}
//~^ ERROR `?const` is not permitted in `impl Trait`
//~| ERROR `?const` on trait bounds is not yet implemented

fn rpit_assoc_bound() -> impl IntoIterator<Item: ?const T> { Some(S) }
//~^ ERROR `?const` is not permitted in `impl Trait`
//~| ERROR `?const` on trait bounds is not yet implemented

fn apit_assoc_bound(_: impl IntoIterator<Item: ?const T>) {}
//~^ ERROR `?const` is not permitted in `impl Trait`
//~| ERROR `?const` on trait bounds is not yet implemented

fn main() {}
