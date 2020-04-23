#![no_std]

use sp_runtime::generic;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Header;

type BlockNumber = u128;
type MyHeader = generic::Header<BlockNumber, BlakeTwo256>;

pub fn dosomething() {
  let h = MyHeader::new(
    0,
    Default::default(),
    Default::default(),
    Default::default(),
    Default::default()
  );

  // This triggers "undefined reference to `ext_hashing_blake2_256_version_1'"
  h.hash();
}
