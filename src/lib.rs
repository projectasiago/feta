#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![allow(unused_variables)]

#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "no_std", feature(alloc))]
#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
mod std {
	pub use alloc::{boxed, string, vec};
	
	pub use core::{cmp, convert, fmt, iter, mem, ops, option, result, slice, str};
	
	pub mod prelude {
		pub use core::prelude as v1;
	}
}

use std::vec::Vec;

pub trait RemoveWhere<T> {
	fn remove_where<F>(&mut self, f: F)
		where F: FnMut(&T) -> bool;
}

impl<T> RemoveWhere<T> for Vec<T> {
	fn remove_where<F>(&mut self, mut f: F)
		where F: FnMut(&T) -> bool
	{
		self.retain(|c: &T| !f(c))
	}
}