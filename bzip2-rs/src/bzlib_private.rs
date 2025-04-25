#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

#[derive(PartialEq)]
#[repr(C)]
pub
struct EState <'a>
{
  pub strm: &'a [crate::bzlib::bz_stream],
  pub mode: i32,
  pub state: i32,
  pub avail_in_expect: u32,
  pub arr1: &'a mut [u32],
  pub arr2: &'a mut [u32],
  pub ftab: &'a mut [u32],
  pub origPtr: i32,
  pub ptr: &'a mut [u32],
  pub block: &'a mut [u8],
  pub zbits_ofs: usize,
  pub workFactor: i32,
  pub state_in_ch: u32,
  pub state_in_len: i32,
  pub rNToGo: i32,
  pub rTPos: i32,
  pub nblock: i32,
  pub nblockMAX: i32,
  pub numZ: i32,
  pub state_out_pos: i32,
  pub nInUse: i32,
  pub inUse: [u8; 256],
  pub unseqToSeq: [u8; 256],
  pub bsBuff: u32,
  pub bsLive: i32,
  pub blockCRC: u32,
  pub combinedCRC: u32,
  pub verbosity: i32,
  pub blockNo: i32,
  pub blockSize100k: i32,
  pub nMTF: i32,
  pub mtfFreq: [i32; 258],
  pub selector: [u8; 18002],
  pub selectorMtf: [u8; 18002],
  pub len: [[u8; 258]; 6],
  pub code: [[i32; 258]; 6],
  pub rfreq: [[i32; 258]; 6],
  pub len_pack: [[u32; 4]; 258]
}
