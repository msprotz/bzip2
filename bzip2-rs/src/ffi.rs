#[derive(PartialEq)]
#[repr(C)]
pub
struct EState
{
  pub strm: *mut crate::bzlib::bz_stream,
  pub mode: i32,
  pub state: i32,
  pub avail_in_expect: u32,
  pub arr1: * mut u32,
  pub arr2: * mut u32,
  pub ftab: * mut u32,
  pub origPtr: i32,
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

const BZ_N_RADIX: usize = 2;
const BZ_N_QSORT: usize = 12;
const BZ_N_SHELL: usize = 18;
const BZ_N_OVERSHOOT: usize = (BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2);


fn mk_state<'a>(s: *mut EState) -> crate::bzlib_private::EState<'a> {
    unsafe {
        let n = 100000 * (*s).blockSize100k;
        crate::bzlib_private::EState {
          strm: std::slice::from_raw_parts_mut((*s).strm, 1),
          mode: (*s).mode,
          state: (*s).state,
          avail_in_expect: (*s).avail_in_expect,
          arr1: std::slice::from_raw_parts_mut((*s).arr1, n as usize),
          arr2: std::slice::from_raw_parts_mut((*s).arr2, n as usize + BZ_N_OVERSHOOT), 
          ftab: std::slice::from_raw_parts_mut((*s).ftab, 65537),
          origPtr: (*s).origPtr,
          zbits_ofs: (*s).zbits_ofs,
          workFactor: (*s).workFactor,
          state_in_ch: (*s).state_in_ch,
          state_in_len: (*s).state_in_len,
          rNToGo: (*s).rNToGo,
          rTPos: (*s).rTPos,
          nblock: (*s).nblock,
          nblockMAX: (*s).nblockMAX,
          numZ: (*s).numZ,
          state_out_pos: (*s).state_out_pos,
          nInUse: (*s).nInUse,
          inUse: (*s).inUse,
          unseqToSeq: (*s).unseqToSeq,
          bsBuff: (*s).bsBuff,
          bsLive: (*s).bsLive,
          blockCRC: (*s).blockCRC,
          combinedCRC: (*s).combinedCRC,
          verbosity: (*s).verbosity,
          blockNo: (*s).blockNo,
          blockSize100k: (*s).blockSize100k,
          nMTF: (*s).nMTF,
          mtfFreq: (*s).mtfFreq,
          selector: (*s).selector,
          selectorMtf: (*s).selectorMtf,
          len: (*s).len,
          code: (*s).code,
          rfreq: (*s).rfreq,
          len_pack: (*s).len_pack,
        }
    }
}

fn update_state<'a>(s: *mut EState, s_rs: &'a crate::bzlib_private::EState<'a>) {
    unsafe {
          (*s).mode = s_rs.mode;
          (*s).state = s_rs.state;
          (*s).avail_in_expect = s_rs.avail_in_expect;
          (*s).origPtr = s_rs.origPtr;
          (*s).zbits_ofs = s_rs.zbits_ofs;
          (*s).workFactor = s_rs.workFactor;
          (*s).state_in_ch = s_rs.state_in_ch;
          (*s).state_in_len = s_rs.state_in_len;
          (*s).rNToGo = s_rs.rNToGo;
          (*s).rTPos = s_rs.rTPos;
          (*s).nblock = s_rs.nblock;
          (*s).nblockMAX = s_rs.nblockMAX;
          (*s).numZ = s_rs.numZ;
          (*s).state_out_pos = s_rs.state_out_pos;
          (*s).nInUse = s_rs.nInUse;
          (*s).inUse = s_rs.inUse;
          (*s).unseqToSeq = s_rs.unseqToSeq;
          (*s).bsBuff = s_rs.bsBuff;
          (*s).bsLive = s_rs.bsLive;
          (*s).blockCRC = s_rs.blockCRC;
          (*s).combinedCRC = s_rs.combinedCRC;
          (*s).verbosity = s_rs.verbosity;
          (*s).blockNo = s_rs.blockNo;
          (*s).blockSize100k = s_rs.blockSize100k;
          (*s).nMTF = s_rs.nMTF;
          (*s).mtfFreq = s_rs.mtfFreq;
          (*s).selector = s_rs.selector;
          (*s).selectorMtf = s_rs.selectorMtf;
          (*s).len = s_rs.len;
          (*s).code = s_rs.code;
          (*s).rfreq = s_rs.rfreq;
          (*s).len_pack = s_rs.len_pack;
    }
}

#[no_mangle]
pub extern "C" fn BZ2_compressBlock(s: *mut EState, is_last_block: u8) {
    let mut s_rs = [ mk_state(s) ];
    crate::compress::BZ2_compressBlock(&mut s_rs, is_last_block);
    update_state(s, &s_rs[0]);
}

const BZ_MAX_CODE_LEN: usize = 23;

#[no_mangle]
pub extern "C" fn BZ2_hbCreateDecodeTables(limit: *mut i32,
  base: *mut i32,
  perm: *mut i32,
  length: *const u8,
  minLen: i32,
  maxLen: i32,
  alphaSize: i32
) {
    let limit = unsafe { std::slice::from_raw_parts_mut(limit, BZ_MAX_CODE_LEN) };
    let base = unsafe { std::slice::from_raw_parts_mut(base, BZ_MAX_CODE_LEN) };
    let perm = unsafe { std::slice::from_raw_parts_mut(perm, (maxLen as usize - minLen as usize + 1) * alphaSize as usize) };
    let length = unsafe { std::slice::from_raw_parts(length, alphaSize as usize) };
    crate::huffman::BZ2_hbCreateDecodeTables(limit, base, perm, length, minLen, maxLen, alphaSize);
}
