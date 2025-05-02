#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn makeMaps_d(s: &mut [crate::bzlib_private::DState])
{
  let mut i: i32;
  (s[0usize]).nInUse = 0i32;
  i = 0i32;
  while
  i < 256i32
  {
    if (s[0usize]).inUse[i as usize] != 0u8
    {
      (s[0usize]).seqToUnseq[(s[0usize]).nInUse as usize] = i as u8;
      (s[0usize]).nInUse = (s[0usize]).nInUse.wrapping_add(1i32)
    };
    i = i.wrapping_add(1i32)
  }
}
