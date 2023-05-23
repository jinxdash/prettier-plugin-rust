#![feature(trait_alias)]

trait A =;
trait A = std::fmt::Debug + Send;

// source: "../../../ext/jinx-rust/tests/samples/features/trait_alias.rs"