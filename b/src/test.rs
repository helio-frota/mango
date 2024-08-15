// NOTE: comment this file, the declaration on lib.rs and the dev-dependencies

// lib.rs
// #[cfg(test)]
// pub mod test;

// Cargo.toml
// # c = { workspace = true }
// # test-context = { workspace = true }

use c::MangoContext;
use test_context::test_context;

use crate::mango_b;

#[test_context(MangoContext)]
#[test]
fn test_mango(ctx: &MangoContext) -> () {
    let m = &ctx.mango;

    mango_b(m.to_string());

    // assert_eq!(1, 2);

}
