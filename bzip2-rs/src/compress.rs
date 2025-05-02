#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn BZ2_bsInitWrite(s: &mut [crate::bzlib_private::EState])
{
  (s[0usize]).bsLive = 0i32;
  (s[0usize]).bsBuff = 0u32
}

pub fn BZ2_compressBlock(s: &mut [crate::bzlib_private::EState], is_last_block: u8)
{
  if (s[0usize]).nblock > 0i32
  {
    (s[0usize]).blockCRC = ! (s[0usize]).blockCRC;
    (s[0usize]).combinedCRC =
        (s[0usize]).combinedCRC.wrapping_shl(1u32) | (s[0usize]).combinedCRC.wrapping_shr(31u32);
    (s[0usize]).combinedCRC ^= (s[0usize]).blockCRC;
    if (s[0usize]).blockNo > 1i32 { (s[0usize]).numZ = 0i32 };
    if (s[0usize]).verbosity >= 2i32 { () };
    crate::blocksort::BZ2_blockSort(s)
  };
  (s[0usize]).zbits_ofs = (s[0usize]).nblock as usize;
  if (s[0usize]).blockNo == 1i32
  {
    BZ2_bsInitWrite(s);
    bsPutUChar(s, 66u8);
    bsPutUChar(s, 90u8);
    bsPutUChar(s, 104u8);
    bsPutUChar(s, 48i32.wrapping_add((s[0usize]).blockSize100k) as u8)
  };
  if (s[0usize]).nblock > 0i32
  {
    bsPutUChar(s, 49u8);
    bsPutUChar(s, 65u8);
    bsPutUChar(s, 89u8);
    bsPutUChar(s, 38u8);
    bsPutUChar(s, 83u8);
    bsPutUChar(s, 89u8);
    bsPutUInt32(s, (s[0usize]).blockCRC);
    bsW(s, 1i32, 0u32);
    bsW(s, 24i32, (s[0usize]).origPtr as u32);
    generateMTFValues(s);
    sendMTFValues(s)
  };
  if is_last_block != 0u8
  {
    bsPutUChar(s, 23u8);
    bsPutUChar(s, 114u8);
    bsPutUChar(s, 69u8);
    bsPutUChar(s, 56u8);
    bsPutUChar(s, 80u8);
    bsPutUChar(s, 144u8);
    bsPutUInt32(s, (s[0usize]).combinedCRC);
    if (s[0usize]).verbosity >= 2i32 { () };
    bsFinishWrite(s)
  }
}

pub fn bsFinishWrite(s: &mut [crate::bzlib_private::EState])
{
  while
  (s[0usize]).bsLive > 0i32
  {
    let zbits: &mut [u8] =
        &mut crate::scylla_glue::scylla_u8_of_u32((s[0usize]).arr2)[(s[0usize]).zbits_ofs..];
    zbits[(s[0usize]).numZ as usize] = (s[0usize]).bsBuff.wrapping_shr(24u32) as u8;
    (s[0usize]).numZ = (s[0usize]).numZ.wrapping_add(1i32);
    (s[0usize]).bsBuff = (s[0usize]).bsBuff.wrapping_shl(8u32);
    (s[0usize]).bsLive = (s[0usize]).bsLive.wrapping_sub(8i32)
  }
}

pub fn bsPutUChar(s: &mut [crate::bzlib_private::EState], c: u8) { bsW(s, 8i32, c as u32) }

pub fn bsPutUInt32(s: &mut [crate::bzlib_private::EState], u: u32)
{
  bsW(s, 8i32, (u.wrapping_shr(24u32) as u64 & 255u64) as u32);
  bsW(s, 8i32, (u.wrapping_shr(16u32) as u64 & 255u64) as u32);
  bsW(s, 8i32, (u.wrapping_shr(8u32) as u64 & 255u64) as u32);
  bsW(s, 8i32, (u as u64 & 255u64) as u32)
}

#[inline] pub fn bsW(s: &mut [crate::bzlib_private::EState], n: i32, v: u32)
{
  while
  (s[0usize]).bsLive >= 8i32
  {
    let zbits: &mut [u8] =
        &mut crate::scylla_glue::scylla_u8_of_u32((s[0usize]).arr2)[(s[0usize]).zbits_ofs..];
    zbits[(s[0usize]).numZ as usize] = (s[0usize]).bsBuff.wrapping_shr(24u32) as u8;
    (s[0usize]).numZ = (s[0usize]).numZ.wrapping_add(1i32);
    (s[0usize]).bsBuff = (s[0usize]).bsBuff.wrapping_shl(8u32);
    (s[0usize]).bsLive = (s[0usize]).bsLive.wrapping_sub(8i32)
  };
  (s[0usize]).bsBuff |=
      v.wrapping_shl(32i32.wrapping_sub((s[0usize]).bsLive).wrapping_sub(n) as u32);
  (s[0usize]).bsLive = (s[0usize]).bsLive.wrapping_add(n)
}

pub fn generateMTFValues(s: &mut [crate::bzlib_private::EState])
{
  let mut yy: [u8; 256] = [0u8; 256usize];
  let mut i: i32;
  let mut j: i32;
  let mut zPend: i32;
  let mut wr: i32;
  let mut EOB: i32;
  makeMaps_e(s);
  let block: &[u8] = crate::scylla_glue::scylla_u8_of_u32((s[0usize]).arr2);
  EOB = (s[0usize]).nInUse.wrapping_add(1i32);
  {
    i = 0i32;
    while
    i <= EOB
    {
      (s[0usize]).mtfFreq[i as usize] = 0i32;
      i = i.wrapping_add(1i32)
    }
  };
  wr = 0i32;
  zPend = 0i32;
  {
    i = 0i32;
    while
    i < (s[0usize]).nInUse
    {
      yy[i as usize] = i as u8;
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < (s[0usize]).nblock
    {
      {
        let mut ll_i: u8;
        j = ((s[0usize]).arr1[i as usize]).wrapping_sub(1u32) as i32;
        if j < 0i32 { j = j.wrapping_add((s[0usize]).nblock) };
        ll_i = (s[0usize]).unseqToSeq[block[j as usize] as usize];
        if yy[0usize] == ll_i
        { zPend = zPend.wrapping_add(1i32) }
        else
        {
          if zPend > 0i32
          {
            zPend = zPend.wrapping_sub(1i32);
            while
            1i32 as u8 != 0u8
            {
              if zPend & 1i32 != 0i32
              {
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] = 1u16;
                wr = wr.wrapping_add(1i32);
                (s[0usize]).mtfFreq[1usize] = ((s[0usize]).mtfFreq[1usize]).wrapping_add(1i32)
              }
              else
              {
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] = 0u16;
                wr = wr.wrapping_add(1i32);
                (s[0usize]).mtfFreq[0usize] = ((s[0usize]).mtfFreq[0usize]).wrapping_add(1i32)
              };
              if zPend < 2i32 { break };
              zPend = zPend.wrapping_sub(2i32).wrapping_div(2i32)
            };
            zPend = 0i32
          };
          let mut rtmp: u8;
          let mut yy_j: usize;
          let mut rll_i: u8;
          rtmp = yy[1usize];
          yy[1usize] = yy[0usize];
          yy_j = 1usize;
          rll_i = ll_i;
          while
          rll_i != rtmp
          {
            let mut rtmp2: u8;
            yy_j = yy_j.wrapping_add(1usize);
            rtmp2 = rtmp;
            rtmp = yy[yy_j];
            yy[yy_j] = rtmp2
          };
          yy[0usize] = rtmp;
          j = yy_j as i32;
          crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] =
              j.wrapping_add(1i32) as u16;
          wr = wr.wrapping_add(1i32);
          (s[0usize]).mtfFreq[j.wrapping_add(1i32) as usize] =
              ((s[0usize]).mtfFreq[j.wrapping_add(1i32) as usize]).wrapping_add(1i32)
        }
      };
      i = i.wrapping_add(1i32)
    }
  };
  if zPend > 0i32
  {
    zPend = zPend.wrapping_sub(1i32);
    while
    1i32 as u8 != 0u8
    {
      if zPend & 1i32 != 0i32
      {
        crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] = 1u16;
        wr = wr.wrapping_add(1i32);
        (s[0usize]).mtfFreq[1usize] = ((s[0usize]).mtfFreq[1usize]).wrapping_add(1i32)
      }
      else
      {
        crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] = 0u16;
        wr = wr.wrapping_add(1i32);
        (s[0usize]).mtfFreq[0usize] = ((s[0usize]).mtfFreq[0usize]).wrapping_add(1i32)
      };
      if zPend < 2i32 { break };
      zPend = zPend.wrapping_sub(2i32).wrapping_div(2i32)
    };
    zPend = 0i32
  };
  crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[wr as usize] = EOB as u16;
  wr = wr.wrapping_add(1i32);
  (s[0usize]).mtfFreq[EOB as usize] = ((s[0usize]).mtfFreq[EOB as usize]).wrapping_add(1i32);
  (s[0usize]).nMTF = wr
}

pub fn makeMaps_e(s: &mut [crate::bzlib_private::EState])
{
  let mut i: i32;
  (s[0usize]).nInUse = 0i32;
  i = 0i32;
  while
  i < 256i32
  {
    if (s[0usize]).inUse[i as usize] != 0u8
    {
      (s[0usize]).unseqToSeq[i as usize] = (s[0usize]).nInUse as u8;
      (s[0usize]).nInUse = (s[0usize]).nInUse.wrapping_add(1i32)
    };
    i = i.wrapping_add(1i32)
  }
}

pub fn sendMTFValues(s: &mut [crate::bzlib_private::EState])
{
  let mut v: i32;
  let mut t: i32;
  let mut i: i32;
  let mut j: i32;
  let mut gs: i32;
  let mut ge: i32;
  let mut totc: i32;
  let mut bt: i32;
  let mut bc: i32;
  let mut iter: i32;
  let mut nSelectors: i32 = 0i32;
  let mut alphaSize: i32;
  let mut minLen: i32;
  let mut maxLen: i32;
  let mut selCtr: i32;
  let mut nGroups: i32;
  let mut nBytes: i32;
  let mut cost: [u16; 6] = [0u16; 6usize];
  let mut fave: [i32; 6] = [0i32; 6usize];
  if (s[0usize]).verbosity >= 3i32 { () };
  alphaSize = (s[0usize]).nInUse.wrapping_add(2i32);
  {
    t = 0i32;
    while
    t < 6i32
    {
      {
        v = 0i32;
        while
        v < alphaSize
        {
          (s[0usize]).len[t as usize][v as usize] = 15u8;
          v = v.wrapping_add(1i32)
        }
      };
      t = t.wrapping_add(1i32)
    }
  };
  if (s[0usize]).nMTF < 200i32
  { nGroups = 2i32 }
  else if (s[0usize]).nMTF < 600i32
  { nGroups = 3i32 }
  else if (s[0usize]).nMTF < 1200i32
  { nGroups = 4i32 }
  else if (s[0usize]).nMTF < 2400i32 { nGroups = 5i32 } else { nGroups = 6i32 };
  {
    let mut nPart: i32;
    let mut remF: i32;
    let mut tFreq: i32;
    let mut aFreq: i32;
    nPart = nGroups;
    remF = (s[0usize]).nMTF;
    gs = 0i32;
    while
    nPart > 0i32
    {
      tFreq = remF.wrapping_div(nPart);
      ge = gs.wrapping_sub(1i32);
      aFreq = 0i32;
      while
      aFreq < tFreq && ge < alphaSize.wrapping_sub(1i32)
      {
        ge = ge.wrapping_add(1i32);
        aFreq = aFreq.wrapping_add((s[0usize]).mtfFreq[ge as usize])
      };
      if
      ge > gs && nPart != nGroups && nPart != 1i32
      &&
      nGroups.wrapping_sub(nPart).wrapping_rem(2i32) == 1i32
      {
        aFreq = aFreq.wrapping_sub((s[0usize]).mtfFreq[ge as usize]);
        ge = ge.wrapping_sub(1i32)
      };
      if (s[0usize]).verbosity >= 3i32 { () };
      {
        v = 0i32;
        while
        v < alphaSize
        {
          if v >= gs && v <= ge
          { (s[0usize]).len[nPart.wrapping_sub(1i32) as usize][v as usize] = 0u8 }
          else
          { (s[0usize]).len[nPart.wrapping_sub(1i32) as usize][v as usize] = 15u8 };
          v = v.wrapping_add(1i32)
        }
      };
      nPart = nPart.wrapping_sub(1i32);
      gs = ge.wrapping_add(1i32);
      remF = remF.wrapping_sub(aFreq)
    }
  };
  {
    iter = 0i32;
    while
    iter < 4i32
    {
      {
        {
          t = 0i32;
          while
          t < nGroups
          {
            fave[t as usize] = 0i32;
            t = t.wrapping_add(1i32)
          }
        };
        {
          t = 0i32;
          while
          t < nGroups
          {
            {
              v = 0i32;
              while
              v < alphaSize
              {
                (s[0usize]).rfreq[t as usize][v as usize] = 0i32;
                v = v.wrapping_add(1i32)
              }
            };
            t = t.wrapping_add(1i32)
          }
        };
        if nGroups == 6i32
        {
          v = 0i32;
          while
          v < alphaSize
          {
            {
              (s[0usize]).len_pack[v as usize][0usize] =
                  (((s[0usize]).len[1usize][v as usize]).wrapping_shl(16u32)
                  |
                  (s[0usize]).len[0usize][v as usize])
                  as
                  u32;
              (s[0usize]).len_pack[v as usize][1usize] =
                  (((s[0usize]).len[3usize][v as usize]).wrapping_shl(16u32)
                  |
                  (s[0usize]).len[2usize][v as usize])
                  as
                  u32;
              (s[0usize]).len_pack[v as usize][2usize] =
                  (((s[0usize]).len[5usize][v as usize]).wrapping_shl(16u32)
                  |
                  (s[0usize]).len[4usize][v as usize])
                  as
                  u32
            };
            v = v.wrapping_add(1i32)
          }
        };
        nSelectors = 0i32;
        totc = 0i32;
        gs = 0i32;
        while
        1i32 as u8 != 0u8
        {
          if gs >= (s[0usize]).nMTF { break };
          ge = gs.wrapping_add(50i32).wrapping_sub(1i32);
          if ge >= (s[0usize]).nMTF { ge = (s[0usize]).nMTF.wrapping_sub(1i32) };
          {
            t = 0i32;
            while
            t < nGroups
            {
              cost[t as usize] = 0u16;
              t = t.wrapping_add(1i32)
            }
          };
          if nGroups == 6i32 && 50i32 == ge.wrapping_sub(gs).wrapping_add(1i32)
          {
            let mut cost01: u32;
            let mut cost23: u32;
            let mut cost45: u32;
            let mut icv: u16;
            {
              cost01 = 0u32;
              cost23 = 0u32;
              cost45 = 0u32
            };
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(0i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(1i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(2i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(3i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(4i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(5i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(6i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(7i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(8i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(9i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(10i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(11i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(12i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(13i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(14i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(15i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(16i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(17i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(18i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(19i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(20i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(21i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(22i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(23i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(24i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(25i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(26i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(27i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(28i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(29i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(30i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(31i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(32i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(33i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(34i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(35i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(36i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(37i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(38i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(39i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(40i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(41i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(42i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(43i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(44i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(45i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(46i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(47i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(48i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            icv =
                crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(49i32)
                as
                usize];
            cost01 = cost01.wrapping_add((s[0usize]).len_pack[icv as usize][0usize]);
            cost23 = cost23.wrapping_add((s[0usize]).len_pack[icv as usize][1usize]);
            cost45 = cost45.wrapping_add((s[0usize]).len_pack[icv as usize][2usize]);
            cost[0usize] = (cost01 & 65535u32) as u16;
            cost[1usize] = cost01.wrapping_shr(16u32) as u16;
            cost[2usize] = (cost23 & 65535u32) as u16;
            cost[3usize] = cost23.wrapping_shr(16u32) as u16;
            cost[4usize] = (cost45 & 65535u32) as u16;
            cost[5usize] = cost45.wrapping_shr(16u32) as u16
          }
          else
          {
            i = gs;
            while
            i <= ge
            {
              {
                let icv: u16 = crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[i as usize];
                t = 0i32;
                while
                t < nGroups
                {
                  cost[t as usize] =
                      (cost[t as usize]).wrapping_add(
                        (s[0usize]).len[t as usize][icv as usize] as u16
                      );
                  t = t.wrapping_add(1i32)
                }
              };
              i = i.wrapping_add(1i32)
            }
          };
          bc = 999999999i32;
          bt = 0i32.wrapping_sub(1i32);
          {
            t = 0i32;
            while
            t < nGroups
            {
              if (cost[t as usize] as u32) < bc as u32
              {
                bc = cost[t as usize] as i32;
                bt = t
              };
              t = t.wrapping_add(1i32)
            }
          };
          totc = totc.wrapping_add(bc);
          fave[bt as usize] = (fave[bt as usize]).wrapping_add(1i32);
          (s[0usize]).selector[nSelectors as usize] = bt as u8;
          nSelectors = nSelectors.wrapping_add(1i32);
          if nGroups == 6i32 && 50i32 == ge.wrapping_sub(gs).wrapping_add(1i32)
          {
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              0i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(0i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              1i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(1i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              2i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(2i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              3i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(3i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              4i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(4i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              5i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(5i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              6i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(6i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              7i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(7i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              8i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(8i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              9i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(9i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              10i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(10i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              11i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(11i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              12i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(12i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              13i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(13i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              14i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(14i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              15i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(15i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              16i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(16i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              17i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(17i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              18i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(18i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              19i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(19i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              20i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(20i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              21i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(21i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              22i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(22i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              23i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(23i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              24i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(24i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              25i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(25i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              26i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(26i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              27i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(27i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              28i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(28i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              29i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(29i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              30i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(30i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              31i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(31i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              32i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(32i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              33i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(33i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              34i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(34i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              35i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(35i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              36i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(36i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              37i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(37i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              38i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(38i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              39i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(39i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              40i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(40i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              41i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(41i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              42i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(42i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              43i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(43i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              44i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(44i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              45i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(45i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              46i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(46i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              47i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(47i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              48i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(48i32) as usize]
                as
                usize]).wrapping_add(1i32);
            (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(
              49i32
            )
            as
            usize]
            as
            usize] =
                ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                  (s[0usize]).arr1
                )[gs.wrapping_add(49i32) as usize]
                as
                usize]).wrapping_add(1i32)
          }
          else
          {
            i = gs;
            while
            i <= ge
            {
              (s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[i
              as
              usize]
              as
              usize] =
                  ((s[0usize]).rfreq[bt as usize][crate::scylla_glue::scylla_u16_of_u32(
                    (s[0usize]).arr1
                  )[i as usize]
                  as
                  usize]).wrapping_add(1i32);
              i = i.wrapping_add(1i32)
            }
          };
          gs = ge.wrapping_add(1i32)
        };
        if (s[0usize]).verbosity >= 3i32
        {
          t = 0i32;
          while t < nGroups { t = t.wrapping_add(1i32) }
        };
        t = 0i32;
        while
        t < nGroups
        {
          crate::huffman::BZ2_hbMakeCodeLengths(
            &mut (s[0usize]).len[t as usize][0usize..],
            &(s[0usize]).rfreq[t as usize][0usize..],
            alphaSize,
            17i32
          );
          t = t.wrapping_add(1i32)
        }
      };
      iter = iter.wrapping_add(1i32)
    }
  };
  {
    let mut pos: [u8; 6] = [0u8; 6usize];
    let mut ll_i: u8;
    let mut tmp2: u8;
    let mut tmp: u8;
    {
      i = 0i32;
      while
      i < nGroups
      {
        pos[i as usize] = i as u8;
        i = i.wrapping_add(1i32)
      }
    };
    i = 0i32;
    while
    i < nSelectors
    {
      {
        ll_i = (s[0usize]).selector[i as usize];
        j = 0i32;
        tmp = pos[j as usize];
        while
        ll_i != tmp
        {
          j = j.wrapping_add(1i32);
          tmp2 = tmp;
          tmp = pos[j as usize];
          pos[j as usize] = tmp2
        };
        pos[0usize] = tmp;
        (s[0usize]).selectorMtf[i as usize] = j as u8
      };
      i = i.wrapping_add(1i32)
    }
  };
  {
    t = 0i32;
    while
    t < nGroups
    {
      {
        minLen = 32i32;
        maxLen = 0i32;
        {
          i = 0i32;
          while
          i < alphaSize
          {
            {
              if (s[0usize]).len[t as usize][i as usize] as u32 > maxLen as u32
              { maxLen = (s[0usize]).len[t as usize][i as usize] as i32 };
              if ((s[0usize]).len[t as usize][i as usize] as u32) < minLen as u32
              { minLen = (s[0usize]).len[t as usize][i as usize] as i32 }
            };
            i = i.wrapping_add(1i32)
          }
        };
        crate::huffman::BZ2_hbAssignCodes(
          &mut (s[0usize]).code[t as usize][0usize..],
          &(s[0usize]).len[t as usize][0usize..],
          minLen,
          maxLen,
          alphaSize
        )
      };
      t = t.wrapping_add(1i32)
    }
  };
  {
    let mut inUse16: [u8; 16] = [0u8; 16usize];
    {
      i = 0i32;
      while
      i < 16i32
      {
        {
          inUse16[i as usize] = 0i32 as u8;
          j = 0i32;
          while
          j < 16i32
          {
            if (s[0usize]).inUse[i.wrapping_mul(16i32).wrapping_add(j) as usize] != 0u8
            { inUse16[i as usize] = 1i32 as u8 };
            j = j.wrapping_add(1i32)
          }
        };
        i = i.wrapping_add(1i32)
      }
    };
    nBytes = (s[0usize]).numZ;
    {
      i = 0i32;
      while
      i < 16i32
      {
        if inUse16[i as usize] != 0u8 { bsW(s, 1i32, 1u32) } else { bsW(s, 1i32, 0u32) };
        i = i.wrapping_add(1i32)
      }
    };
    {
      i = 0i32;
      while
      i < 16i32
      {
        if inUse16[i as usize] != 0u8
        {
          j = 0i32;
          while
          j < 16i32
          {
            if (s[0usize]).inUse[i.wrapping_mul(16i32).wrapping_add(j) as usize] != 0u8
            { bsW(s, 1i32, 1u32) }
            else
            { bsW(s, 1i32, 0u32) };
            j = j.wrapping_add(1i32)
          }
        };
        i = i.wrapping_add(1i32)
      }
    };
    if (s[0usize]).verbosity >= 3i32 { () }
  };
  nBytes = (s[0usize]).numZ;
  bsW(s, 3i32, nGroups as u32);
  bsW(s, 15i32, nSelectors as u32);
  {
    i = 0i32;
    while
    i < nSelectors
    {
      {
        {
          j = 0i32;
          while
          (j as u32) < (s[0usize]).selectorMtf[i as usize] as u32
          {
            bsW(s, 1i32, 1u32);
            j = j.wrapping_add(1i32)
          }
        };
        bsW(s, 1i32, 0u32)
      };
      i = i.wrapping_add(1i32)
    }
  };
  if (s[0usize]).verbosity >= 3i32 { () };
  nBytes = (s[0usize]).numZ;
  {
    t = 0i32;
    while
    t < nGroups
    {
      {
        let mut curr: i32 = (s[0usize]).len[t as usize][0usize] as i32;
        bsW(s, 5i32, curr as u32);
        i = 0i32;
        while
        i < alphaSize
        {
          {
            while
            (curr as u32) < (s[0usize]).len[t as usize][i as usize] as u32
            {
              bsW(s, 2i32, 2u32);
              curr = curr.wrapping_add(1i32)
            };
            while
            curr as u32 > (s[0usize]).len[t as usize][i as usize] as u32
            {
              bsW(s, 2i32, 3u32);
              curr = curr.wrapping_sub(1i32)
            };
            bsW(s, 1i32, 0u32)
          };
          i = i.wrapping_add(1i32)
        }
      };
      t = t.wrapping_add(1i32)
    }
  };
  if (s[0usize]).verbosity >= 3i32 { () };
  nBytes = (s[0usize]).numZ;
  selCtr = 0i32;
  gs = 0i32;
  while
  1i32 as u8 != 0u8
  {
    if gs >= (s[0usize]).nMTF { break };
    ge = gs.wrapping_add(50i32).wrapping_sub(1i32);
    if ge >= (s[0usize]).nMTF { ge = (s[0usize]).nMTF.wrapping_sub(1i32) };
    if nGroups == 6i32 && 50i32 == ge.wrapping_sub(gs).wrapping_add(1i32)
    {
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(0i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(1i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(2i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(3i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(4i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(5i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(6i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(7i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(8i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(9i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(10i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(11i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(12i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(13i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(14i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(15i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(16i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(17i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(18i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(19i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(20i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(21i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(22i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(23i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(24i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(25i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(26i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(27i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(28i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(29i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(30i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(31i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(32i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(33i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(34i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(35i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(36i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(37i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(38i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(39i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(40i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(41i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(42i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(43i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(44i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(45i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(46i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(47i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      {
        let mtfv_i: u16 =
            crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(48i32) as usize];
        let s_len_sel_selCtr: &[u8] =
            &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        let s_code_sel_selCtr: &[i32] =
            &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
        bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
      };
      let mtfv_i: u16 =
          crate::scylla_glue::scylla_u16_of_u32((s[0usize]).arr1)[gs.wrapping_add(49i32) as usize];
      let s_len_sel_selCtr: &[u8] =
          &(s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
      let s_code_sel_selCtr: &[i32] =
          &(s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][0usize..];
      bsW(s, s_len_sel_selCtr[mtfv_i as usize] as i32, s_code_sel_selCtr[mtfv_i as usize] as u32)
    }
    else
    {
      i = gs;
      while
      i <= ge
      {
        {
          let n: i32 =
              (s[0usize]).len[(s[0usize]).selector[selCtr as usize] as usize][crate::scylla_glue::scylla_u16_of_u32(
                (s[0usize]).arr1
              )[i as usize]
              as
              usize]
              as
              i32;
          let v0: u32 =
              (s[0usize]).code[(s[0usize]).selector[selCtr as usize] as usize][crate::scylla_glue::scylla_u16_of_u32(
                (s[0usize]).arr1
              )[i as usize]
              as
              usize]
              as
              u32;
          bsW(s, n, v0)
        };
        i = i.wrapping_add(1i32)
      }
    };
    gs = ge.wrapping_add(1i32);
    selCtr = selCtr.wrapping_add(1i32)
  };
  if (s[0usize]).verbosity >= 3i32 { () }
}
