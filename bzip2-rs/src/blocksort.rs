#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]

pub fn BZ2_blockSort(s: &mut [crate::bzlib_private::EState])
{
  let ptr: &mut [u32] = (s[0usize]).ptr;
  let block: &mut [u8] = (s[0usize]).block;
  let ftab: &mut [u32] = (s[0usize]).ftab;
  let nblock: i32 = (s[0usize]).nblock;
  let verb: i32 = (s[0usize]).verbosity;
  let mut wfact: i32 = (s[0usize]).workFactor;
  let mut quadrant: &mut [u16] = Default::default();
  let mut budget: i32;
  let mut budgetInit: i32;
  let mut i: i32;
  if nblock < 10000i32
  { fallbackSort((s[0usize]).arr1, (s[0usize]).arr2, ftab, nblock, verb) }
  else
  {
    i = nblock.wrapping_add(2i32.wrapping_add(12i32).wrapping_add(18i32).wrapping_add(2i32));
    if i & 1i32 != 0i32 { i = i.wrapping_add(1i32) };
    quadrant = &mut block[i as usize..];
    if wfact < 1i32 { wfact = 1i32 };
    if wfact > 100i32 { wfact = 100i32 };
    budgetInit = nblock.wrapping_mul(wfact.wrapping_sub(1i32).wrapping_div(3i32));
    budget = budgetInit;
    mainSort(ptr, block, quadrant, ftab, nblock, verb, std::slice::from_slice::<i32>(&budget));
    if verb >= 3i32 { () };
    if budget < 0i32
    {
      if verb >= 2i32 { () };
      fallbackSort((s[0usize]).arr1, (s[0usize]).arr2, ftab, nblock, verb)
    }
  };
  (s[0usize]).origPtr = 0i32.wrapping_sub(1i32);
  i = 0i32;
  while
  i < (s[0usize]).nblock
  {
    if ptr[i as usize] == 0u32
    {
      (s[0usize]).origPtr = i;
      break
    };
    i = i.wrapping_add(1i32)
  }
}

pub fn fallbackQSort3(fmap: &mut [u32], eclass: &[u32], loSt: i32, hiSt: i32)
{
  let mut unLo: i32;
  let mut unHi: i32;
  let mut ltLo: i32;
  let mut gtHi: i32;
  let mut n: i32;
  let mut m: i32;
  let mut sp: i32;
  let mut lo: i32;
  let mut hi: i32;
  let mut med: u32;
  let mut r: u32;
  let mut r3: u32;
  let mut stackLo: [i32; 100] = Default::default();
  let mut stackHi: [i32; 100] = Default::default();
  r = 0u32;
  sp = 0i32;
  {
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp = sp.wrapping_add(1i32)
  };
  while
  sp > 0i32
  {
    {
      sp = sp.wrapping_sub(1i32);
      lo = stackLo[sp as usize];
      hi = stackHi[sp as usize]
    };
    if hi.wrapping_sub(lo) < 10i32
    {
      fallbackSimpleSort(fmap, eclass, lo, hi);
      continue
    };
    r = r.wrapping_mul(7621u32).wrapping_add(1u32).wrapping_rem(32768u32);
    r3 = r.wrapping_rem(3u32);
    if r3 == 0u32
    { med = eclass[fmap[lo as usize] as usize] }
    else if r3 == 1u32
    { med = eclass[fmap[lo.wrapping_add(hi).wrapping_shr(1u32) as usize] as usize] }
    else
    { med = eclass[fmap[hi as usize] as usize] };
    unLo = (ltLo = lo) as i32;
    unHi = (gtHi = hi) as i32;
    while
    true
    {
      while
      true
      {
        if unLo > unHi { break };
        n = (eclass[fmap[unLo as usize] as usize] as i32).wrapping_sub(med as i32);
        if n == 0i32
        {
          {
            let zztmp: i32 = fmap[unLo as usize] as i32;
            fmap[unLo as usize] = fmap[ltLo as usize];
            fmap[ltLo as usize] = zztmp as u32
          };
          ltLo = ltLo.wrapping_add(1i32);
          unLo = unLo.wrapping_add(1i32);
          continue
        };
        if n > 0i32 { break };
        unLo = unLo.wrapping_add(1i32)
      };
      while
      true
      {
        if unLo > unHi { break };
        n = (eclass[fmap[unHi as usize] as usize] as i32).wrapping_sub(med as i32);
        if n == 0i32
        {
          {
            let zztmp: i32 = fmap[unHi as usize] as i32;
            fmap[unHi as usize] = fmap[gtHi as usize];
            fmap[gtHi as usize] = zztmp as u32
          };
          gtHi = gtHi.wrapping_sub(1i32);
          unHi = unHi.wrapping_sub(1i32);
          continue
        };
        if n < 0i32 { break };
        unHi = unHi.wrapping_sub(1i32)
      };
      if unLo > unHi { break };
      {
        let zztmp: i32 = fmap[unLo as usize] as i32;
        fmap[unLo as usize] = fmap[unHi as usize];
        fmap[unHi as usize] = zztmp as u32
      };
      unLo = unLo.wrapping_add(1i32);
      unHi = unHi.wrapping_sub(1i32)
    };
    if gtHi < ltLo { continue };
    n =
        if ltLo.wrapping_sub(lo) < unLo.wrapping_sub(ltLo)
        { ltLo.wrapping_sub(lo) }
        else
        { unLo.wrapping_sub(ltLo) };
    {
      let mut yyp1: i32 = lo;
      let mut yyp2: i32 = unLo.wrapping_sub(n);
      let mut yyn: i32 = n;
      while
      yyn > 0i32
      {
        {
          let zztmp: i32 = fmap[yyp1 as usize] as i32;
          fmap[yyp1 as usize] = fmap[yyp2 as usize];
          fmap[yyp2 as usize] = zztmp as u32
        };
        yyp1 = yyp1.wrapping_add(1i32);
        yyp2 = yyp2.wrapping_add(1i32);
        yyn = yyn.wrapping_sub(1i32)
      }
    };
    m =
        if hi.wrapping_sub(gtHi) < gtHi.wrapping_sub(unHi)
        { hi.wrapping_sub(gtHi) }
        else
        { gtHi.wrapping_sub(unHi) };
    {
      let mut yyp1: i32 = unLo;
      let mut yyp2: i32 = hi.wrapping_sub(m).wrapping_add(1i32);
      let mut yyn: i32 = m;
      while
      yyn > 0i32
      {
        {
          let zztmp: i32 = fmap[yyp1 as usize] as i32;
          fmap[yyp1 as usize] = fmap[yyp2 as usize];
          fmap[yyp2 as usize] = zztmp as u32
        };
        yyp1 = yyp1.wrapping_add(1i32);
        yyp2 = yyp2.wrapping_add(1i32);
        yyn = yyn.wrapping_sub(1i32)
      }
    };
    n = lo.wrapping_add(unLo).wrapping_sub(ltLo).wrapping_sub(1i32);
    m = hi.wrapping_sub(gtHi.wrapping_sub(unHi)).wrapping_add(1i32);
    if n.wrapping_sub(lo) > hi.wrapping_sub(m)
    {
      {
        stackLo[sp as usize] = lo;
        stackHi[sp as usize] = n;
        sp = sp.wrapping_add(1i32)
      };
      stackLo[sp as usize] = m;
      stackHi[sp as usize] = hi;
      sp = sp.wrapping_add(1i32)
    }
    else
    {
      {
        stackLo[sp as usize] = m;
        stackHi[sp as usize] = hi;
        sp = sp.wrapping_add(1i32)
      };
      stackLo[sp as usize] = lo;
      stackHi[sp as usize] = n;
      sp = sp.wrapping_add(1i32)
    }
  }
}

#[inline] pub fn fallbackSimpleSort(fmap: &mut [u32], eclass: &[u32], lo: i32, hi: i32)
{
  let mut i: i32;
  let mut j: i32;
  let mut tmp: i32;
  let mut ec_tmp: u32;
  if lo == hi { return () };
  if hi.wrapping_sub(lo) > 3i32
  {
    i = hi.wrapping_sub(4i32);
    while
    i >= lo
    {
      {
        tmp = fmap[i as usize] as i32;
        ec_tmp = eclass[tmp as usize];
        {
          j = i.wrapping_add(4i32);
          while
          j <= hi && ec_tmp > eclass[fmap[j as usize] as usize]
          {
            fmap[j.wrapping_sub(4i32) as usize] = fmap[j as usize];
            j = j.wrapping_add(4i32)
          }
        };
        fmap[j.wrapping_sub(4i32) as usize] = tmp as u32
      };
      i = i.wrapping_sub(1i32)
    }
  };
  i = hi.wrapping_sub(1i32);
  while
  i >= lo
  {
    {
      tmp = fmap[i as usize] as i32;
      ec_tmp = eclass[tmp as usize];
      {
        j = i.wrapping_add(1i32);
        while
        j <= hi && ec_tmp > eclass[fmap[j as usize] as usize]
        {
          fmap[j.wrapping_sub(1i32) as usize] = fmap[j as usize];
          j = j.wrapping_add(1i32)
        }
      };
      fmap[j.wrapping_sub(1i32) as usize] = tmp as u32
    };
    i = i.wrapping_sub(1i32)
  }
}

pub fn fallbackSort(
  fmap: &mut [u32],
  eclass: &mut [u32],
  bhtab: &mut [u32],
  nblock: i32,
  verb: i32
)
{
  let mut ftab: [i32; 257] = Default::default();
  let mut ftabCopy: [i32; 256] = Default::default();
  let mut H: i32;
  let mut i: i32;
  let mut j: i32;
  let mut k: i32;
  let mut l: i32;
  let mut r: i32;
  let mut cc: i32;
  let mut cc1: i32;
  let mut nNotDone: i32;
  let mut nBhtab: i32;
  let eclass8: &mut [u8] = crate::bzlib_private::scylla_u8_of_u32(eclass);
  if verb >= 4i32 { () };
  {
    i = 0i32;
    while
    i < 257i32
    {
      ftab[i as usize] = 0i32;
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < nblock
    {
      ftab[eclass8[i as usize] as usize] = (ftab[eclass8[i as usize] as usize]).wrapping_add(1i32);
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 256i32
    {
      ftabCopy[i as usize] = ftab[i as usize];
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 1i32;
    while
    i < 257i32
    {
      ftab[i as usize] = (ftab[i as usize]).wrapping_add(ftab[i.wrapping_sub(1i32) as usize]);
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < nblock
    {
      {
        j = eclass8[i as usize] as i32;
        k = (ftab[j as usize]).wrapping_sub(1i32);
        ftab[j as usize] = k;
        fmap[k as usize] = i as u32
      };
      i = i.wrapping_add(1i32)
    }
  };
  nBhtab = 2i32.wrapping_add(nblock.wrapping_div(32i32));
  {
    i = 0i32;
    while
    i < nBhtab
    {
      bhtab[i as usize] = 0u32;
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 256i32
    {
      bhtab[(ftab[i as usize]).wrapping_shr(5u32) as usize] |=
          (1i32 as u32).wrapping_shl((ftab[i as usize] & 31i32) as u32);
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 32i32
    {
      {
        bhtab[nblock.wrapping_add(2i32.wrapping_mul(i)).wrapping_shr(5u32) as usize] |=
            (1i32 as u32).wrapping_shl((nblock.wrapping_add(2i32.wrapping_mul(i)) & 31i32) as u32);
        bhtab[nblock.wrapping_add(2i32.wrapping_mul(i)).wrapping_add(1i32).wrapping_shr(5u32)
        as
        usize] &=
            !
            (1i32 as u32).wrapping_shl(
              (nblock.wrapping_add(2i32.wrapping_mul(i)).wrapping_add(1i32) & 31i32) as u32
            )
      };
      i = i.wrapping_add(1i32)
    }
  };
  H = 1i32;
  while
  true
  {
    if verb >= 4i32 { () };
    j = 0i32;
    {
      i = 0i32;
      while
      i < nblock
      {
        {
          if
          bhtab[i.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((i & 31i32) as u32)
          !=
          0u32
          { j = i };
          k = (fmap[i as usize]).wrapping_sub(H as u32) as i32;
          if k < 0i32 { k = k.wrapping_add(nblock) };
          eclass[k as usize] = j as u32
        };
        i = i.wrapping_add(1i32)
      }
    };
    nNotDone = 0i32;
    r = 0i32.wrapping_sub(1i32);
    while
    true
    {
      k = r.wrapping_add(1i32);
      while
      bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32) != 0u32
      &&
      k & 31i32 != 0i32
      { k = k.wrapping_add(1i32) };
      if
      bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32) != 0u32
      {
        while bhtab[k.wrapping_shr(5u32) as usize] == 4294967295u32 { k = k.wrapping_add(32i32) };
        while
        bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32)
        !=
        0u32
        { k = k.wrapping_add(1i32) }
      };
      l = k.wrapping_sub(1i32);
      if l >= nblock { break };
      while
      bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32) == 0u32
      &&
      k & 31i32 != 0i32
      { k = k.wrapping_add(1i32) };
      if
      bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32) == 0u32
      {
        while bhtab[k.wrapping_shr(5u32) as usize] == 0u32 { k = k.wrapping_add(32i32) };
        while
        bhtab[k.wrapping_shr(5u32) as usize] & (1i32 as u32).wrapping_shl((k & 31i32) as u32)
        ==
        0u32
        { k = k.wrapping_add(1i32) }
      };
      r = k.wrapping_sub(1i32);
      if r >= nblock { break };
      if r > l
      {
        nNotDone = nNotDone.wrapping_add(r.wrapping_sub(l).wrapping_add(1i32));
        fallbackQSort3(fmap, eclass, l, r);
        cc = 0i32.wrapping_sub(1i32);
        i = l;
        while
        i <= r
        {
          {
            cc1 = eclass[fmap[i as usize] as usize] as i32;
            if cc != cc1
            {
              bhtab[i.wrapping_shr(5u32) as usize] |= (1i32 as u32).wrapping_shl((i & 31i32) as u32);
              cc = cc1
            }
          };
          i = i.wrapping_add(1i32)
        }
      }
    };
    if verb >= 4i32 { () };
    H = H.wrapping_mul(2i32);
    if H > nblock || nNotDone == 0i32 { break }
  };
  if verb >= 4i32 { () };
  j = 0i32;
  i = 0i32;
  while
  i < nblock
  {
    {
      while ftabCopy[j as usize] == 0i32 { j = j.wrapping_add(1i32) };
      ftabCopy[j as usize] = (ftabCopy[j as usize]).wrapping_sub(1i32);
      eclass8[fmap[i as usize] as usize] = j as u8
    };
    i = i.wrapping_add(1i32)
  }
}

pub const incs: [i32; 14] =
    [1i32, 4i32, 13i32, 40i32, 121i32, 364i32, 1093i32, 3280i32, 9841i32, 29524i32, 88573i32,
        265720i32, 797161i32, 2391484i32];

#[inline] pub fn mainGtU(
  mut i1: u32,
  mut i2: u32,
  block: &[u8],
  quadrant: &[u16],
  nblock: u32,
  budget: &mut [i32]
) ->
    u8
{
  let mut k: i32;
  let mut c1: u8;
  let mut c2: u8;
  let mut s1: u16;
  let mut s2: u16;
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  c1 = block[i1 as usize];
  c2 = block[i2 as usize];
  if c1 != c2 { return (c1 > c2) as u8 };
  i1 = i1.wrapping_add(1u32);
  i2 = i2.wrapping_add(1u32);
  k = nblock.wrapping_add(8u32) as i32;
  {
    {
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      if i1 >= nblock { i1 = i1.wrapping_sub(nblock) };
      if i2 >= nblock { i2 = i2.wrapping_sub(nblock) };
      k = k.wrapping_sub(8i32);
      budget[0usize] = (budget[0usize]).wrapping_sub(1i32)
    };
    while
    k >= 0i32
    {
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      c1 = block[i1 as usize];
      c2 = block[i2 as usize];
      if c1 != c2 { return (c1 > c2) as u8 };
      s1 = quadrant[i1 as usize];
      s2 = quadrant[i2 as usize];
      if s1 != s2 { return (s1 > s2) as u8 };
      i1 = i1.wrapping_add(1u32);
      i2 = i2.wrapping_add(1u32);
      if i1 >= nblock { i1 = i1.wrapping_sub(nblock) };
      if i2 >= nblock { i2 = i2.wrapping_sub(nblock) };
      k = k.wrapping_sub(8i32);
      budget[0usize] = (budget[0usize]).wrapping_sub(1i32)
    }
  };
  return 0i32 as u8
}

pub fn mainQSort3(
  ptr: &mut [u32],
  block: &[u8],
  quadrant: &[u16],
  nblock: i32,
  loSt: i32,
  hiSt: i32,
  dSt: i32,
  budget: &[i32]
)
{
  let mut unLo: i32;
  let mut unHi: i32;
  let mut ltLo: i32;
  let mut gtHi: i32;
  let mut n: i32;
  let mut m: i32;
  let mut med: i32;
  let mut sp: i32;
  let mut lo: i32;
  let mut hi: i32;
  let mut d: i32;
  let mut stackLo: [i32; 100] = Default::default();
  let mut stackHi: [i32; 100] = Default::default();
  let mut stackD: [i32; 100] = Default::default();
  let mut nextLo: [i32; 3] = Default::default();
  let mut nextHi: [i32; 3] = Default::default();
  let mut nextD: [i32; 3] = Default::default();
  sp = 0i32;
  {
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp = sp.wrapping_add(1i32)
  };
  while
  sp > 0i32
  {
    {
      sp = sp.wrapping_sub(1i32);
      lo = stackLo[sp as usize];
      hi = stackHi[sp as usize];
      d = stackD[sp as usize]
    };
    if hi.wrapping_sub(lo) < 20i32 || d > 2i32.wrapping_add(12i32)
    {
      mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
      if budget[0usize] < 0i32 { return () };
      continue
    };
    med =
        mmed3(
          block[(ptr[lo as usize]).wrapping_add(d as u32) as usize],
          block[(ptr[hi as usize]).wrapping_add(d as u32) as usize],
          block[(ptr[lo.wrapping_add(hi).wrapping_shr(1u32) as usize]).wrapping_add(d as u32)
          as
          usize]
        )
        as
        i32;
    unLo = (ltLo = lo) as i32;
    unHi = (gtHi = hi) as i32;
    while
    1i32 as u8 != 0u8
    {
      while
      1i32 as u8 != 0u8
      {
        if unLo > unHi { break };
        n = (block[(ptr[unLo as usize]).wrapping_add(d as u32) as usize] as i32).wrapping_sub(med);
        if n == 0i32
        {
          {
            let zztmp: i32 = ptr[unLo as usize] as i32;
            ptr[unLo as usize] = ptr[ltLo as usize];
            ptr[ltLo as usize] = zztmp as u32
          };
          ltLo = ltLo.wrapping_add(1i32);
          unLo = unLo.wrapping_add(1i32);
          continue
        };
        if n > 0i32 { break };
        unLo = unLo.wrapping_add(1i32)
      };
      while
      1i32 as u8 != 0u8
      {
        if unLo > unHi { break };
        n = (block[(ptr[unHi as usize]).wrapping_add(d as u32) as usize] as i32).wrapping_sub(med);
        if n == 0i32
        {
          {
            let zztmp: i32 = ptr[unHi as usize] as i32;
            ptr[unHi as usize] = ptr[gtHi as usize];
            ptr[gtHi as usize] = zztmp as u32
          };
          gtHi = gtHi.wrapping_sub(1i32);
          unHi = unHi.wrapping_sub(1i32);
          continue
        };
        if n < 0i32 { break };
        unHi = unHi.wrapping_sub(1i32)
      };
      if unLo > unHi { break };
      {
        let zztmp: i32 = ptr[unLo as usize] as i32;
        ptr[unLo as usize] = ptr[unHi as usize];
        ptr[unHi as usize] = zztmp as u32
      };
      unLo = unLo.wrapping_add(1i32);
      unHi = unHi.wrapping_sub(1i32)
    };
    if gtHi < ltLo
    {
      {
        stackLo[sp as usize] = lo;
        stackHi[sp as usize] = hi;
        stackD[sp as usize] = d.wrapping_add(1i32);
        sp = sp.wrapping_add(1i32)
      };
      continue
    };
    n =
        if ltLo.wrapping_sub(lo) < unLo.wrapping_sub(ltLo)
        { ltLo.wrapping_sub(lo) }
        else
        { unLo.wrapping_sub(ltLo) };
    {
      let mut yyp1: i32 = lo;
      let mut yyp2: i32 = unLo.wrapping_sub(n);
      let mut yyn: i32 = n;
      while
      yyn > 0i32
      {
        {
          let zztmp: i32 = ptr[yyp1 as usize] as i32;
          ptr[yyp1 as usize] = ptr[yyp2 as usize];
          ptr[yyp2 as usize] = zztmp as u32
        };
        yyp1 = yyp1.wrapping_add(1i32);
        yyp2 = yyp2.wrapping_add(1i32);
        yyn = yyn.wrapping_sub(1i32)
      }
    };
    m =
        if hi.wrapping_sub(gtHi) < gtHi.wrapping_sub(unHi)
        { hi.wrapping_sub(gtHi) }
        else
        { gtHi.wrapping_sub(unHi) };
    {
      let mut yyp1: i32 = unLo;
      let mut yyp2: i32 = hi.wrapping_sub(m).wrapping_add(1i32);
      let mut yyn: i32 = m;
      while
      yyn > 0i32
      {
        {
          let zztmp: i32 = ptr[yyp1 as usize] as i32;
          ptr[yyp1 as usize] = ptr[yyp2 as usize];
          ptr[yyp2 as usize] = zztmp as u32
        };
        yyp1 = yyp1.wrapping_add(1i32);
        yyp2 = yyp2.wrapping_add(1i32);
        yyn = yyn.wrapping_sub(1i32)
      }
    };
    n = lo.wrapping_add(unLo).wrapping_sub(ltLo).wrapping_sub(1i32);
    m = hi.wrapping_sub(gtHi.wrapping_sub(unHi)).wrapping_add(1i32);
    nextLo[0usize] = lo;
    nextHi[0usize] = n;
    nextD[0usize] = d;
    nextLo[1usize] = m;
    nextHi[1usize] = hi;
    nextD[1usize] = d;
    nextLo[2usize] = n.wrapping_add(1i32);
    nextHi[2usize] = m.wrapping_sub(1i32);
    nextD[2usize] = d.wrapping_add(1i32);
    if
    (nextHi[0usize]).wrapping_sub(nextLo[0usize]) < (nextHi[1usize]).wrapping_sub(nextLo[1usize])
    {
      let mut tz: i32;
      tz = nextLo[0usize];
      nextLo[0usize] = nextLo[1usize];
      nextLo[1usize] = tz;
      tz = nextHi[0usize];
      nextHi[0usize] = nextHi[1usize];
      nextHi[1usize] = tz;
      tz = nextD[0usize];
      nextD[0usize] = nextD[1usize];
      nextD[1usize] = tz
    };
    if
    (nextHi[1usize]).wrapping_sub(nextLo[1usize]) < (nextHi[2usize]).wrapping_sub(nextLo[2usize])
    {
      let mut tz: i32;
      tz = nextLo[1usize];
      nextLo[1usize] = nextLo[2usize];
      nextLo[2usize] = tz;
      tz = nextHi[1usize];
      nextHi[1usize] = nextHi[2usize];
      nextHi[2usize] = tz;
      tz = nextD[1usize];
      nextD[1usize] = nextD[2usize];
      nextD[2usize] = tz
    };
    if
    (nextHi[0usize]).wrapping_sub(nextLo[0usize]) < (nextHi[1usize]).wrapping_sub(nextLo[1usize])
    {
      let mut tz: i32;
      tz = nextLo[0usize];
      nextLo[0usize] = nextLo[1usize];
      nextLo[1usize] = tz;
      tz = nextHi[0usize];
      nextHi[0usize] = nextHi[1usize];
      nextHi[1usize] = tz;
      tz = nextD[0usize];
      nextD[0usize] = nextD[1usize];
      nextD[1usize] = tz
    };
    {
      stackLo[sp as usize] = nextLo[0usize];
      stackHi[sp as usize] = nextHi[0usize];
      stackD[sp as usize] = nextD[0usize];
      sp = sp.wrapping_add(1i32)
    };
    {
      stackLo[sp as usize] = nextLo[1usize];
      stackHi[sp as usize] = nextHi[1usize];
      stackD[sp as usize] = nextD[1usize];
      sp = sp.wrapping_add(1i32)
    };
    stackLo[sp as usize] = nextLo[2usize];
    stackHi[sp as usize] = nextHi[2usize];
    stackD[sp as usize] = nextD[2usize];
    sp = sp.wrapping_add(1i32)
  }
}

pub fn mainSimpleSort(
  ptr: &mut [u32],
  block: &[u8],
  quadrant: &[u16],
  nblock: i32,
  lo: i32,
  hi: i32,
  d: i32,
  budget: &[i32]
)
{
  let mut i: i32;
  let mut j: i32;
  let mut h: i32;
  let mut bigN: i32;
  let mut hp: i32;
  let mut v: u32;
  bigN = hi.wrapping_sub(lo).wrapping_add(1i32);
  if bigN < 2i32 { return () };
  hp = 0i32;
  while incs[hp as usize] < bigN { hp = hp.wrapping_add(1i32) };
  hp = hp.wrapping_sub(1i32);
  while
  hp >= 0i32
  {
    {
      h = incs[hp as usize];
      i = lo.wrapping_add(h);
      while
      1i32 as u8 != 0u8
      {
        if i > hi { break };
        v = ptr[i as usize];
        j = i;
        while
        mainGtU(
          (ptr[j.wrapping_sub(h) as usize]).wrapping_add(d as u32),
          v.wrapping_add(d as u32),
          block,
          quadrant,
          nblock as u32,
          budget
        )
        !=
        0u8
        {
          ptr[j as usize] = ptr[j.wrapping_sub(h) as usize];
          j = j.wrapping_sub(h);
          if j <= lo.wrapping_add(h).wrapping_sub(1i32) { break }
        };
        ptr[j as usize] = v;
        i = i.wrapping_add(1i32);
        if i > hi { break };
        v = ptr[i as usize];
        j = i;
        while
        mainGtU(
          (ptr[j.wrapping_sub(h) as usize]).wrapping_add(d as u32),
          v.wrapping_add(d as u32),
          block,
          quadrant,
          nblock as u32,
          budget
        )
        !=
        0u8
        {
          ptr[j as usize] = ptr[j.wrapping_sub(h) as usize];
          j = j.wrapping_sub(h);
          if j <= lo.wrapping_add(h).wrapping_sub(1i32) { break }
        };
        ptr[j as usize] = v;
        i = i.wrapping_add(1i32);
        if i > hi { break };
        v = ptr[i as usize];
        j = i;
        while
        mainGtU(
          (ptr[j.wrapping_sub(h) as usize]).wrapping_add(d as u32),
          v.wrapping_add(d as u32),
          block,
          quadrant,
          nblock as u32,
          budget
        )
        !=
        0u8
        {
          ptr[j as usize] = ptr[j.wrapping_sub(h) as usize];
          j = j.wrapping_sub(h);
          if j <= lo.wrapping_add(h).wrapping_sub(1i32) { break }
        };
        ptr[j as usize] = v;
        i = i.wrapping_add(1i32);
        if budget[0usize] < 0i32 { return () }
      }
    };
    hp = hp.wrapping_sub(1i32)
  }
}

pub fn mainSort(
  ptr: &mut [u32],
  block: &mut [u8],
  quadrant: &mut [u16],
  ftab: &mut [u32],
  nblock: i32,
  verb: i32,
  budget: &[i32]
)
{
  let mut i: i32;
  let mut j: i32;
  let mut k: i32;
  let mut ss: i32;
  let mut sb: i32;
  let mut runningOrder: [i32; 256] = Default::default();
  let mut bigDone: [u8; 256] = Default::default();
  let mut copyStart: [i32; 256] = Default::default();
  let mut copyEnd: [i32; 256] = Default::default();
  let mut c1: u8;
  let mut numQSorted: i32;
  let mut s: u16;
  if verb >= 4i32 { () };
  {
    i = 65536i32;
    while
    i >= 0i32
    {
      ftab[i as usize] = 0u32;
      i = i.wrapping_sub(1i32)
    }
  };
  j = (block[0usize]).wrapping_shl(8u32) as i32;
  i = nblock.wrapping_sub(1i32);
  {
    while
    i >= 3i32
    {
      {
        quadrant[i as usize] = 0u16;
        j = j.wrapping_shr(8u32) | (block[i as usize] as u16).wrapping_shl(8u32) as i32;
        ftab[j as usize] = (ftab[j as usize]).wrapping_add(1u32);
        quadrant[i.wrapping_sub(1i32) as usize] = 0u16;
        j =
            j.wrapping_shr(8u32)
            |
            (block[i.wrapping_sub(1i32) as usize] as u16).wrapping_shl(8u32) as i32;
        ftab[j as usize] = (ftab[j as usize]).wrapping_add(1u32);
        quadrant[i.wrapping_sub(2i32) as usize] = 0u16;
        j =
            j.wrapping_shr(8u32)
            |
            (block[i.wrapping_sub(2i32) as usize] as u16).wrapping_shl(8u32) as i32;
        ftab[j as usize] = (ftab[j as usize]).wrapping_add(1u32);
        quadrant[i.wrapping_sub(3i32) as usize] = 0u16;
        j =
            j.wrapping_shr(8u32)
            |
            (block[i.wrapping_sub(3i32) as usize] as u16).wrapping_shl(8u32) as i32;
        ftab[j as usize] = (ftab[j as usize]).wrapping_add(1u32)
      };
      i = i.wrapping_sub(4i32)
    }
  };
  {
    while
    i >= 0i32
    {
      {
        quadrant[i as usize] = 0u16;
        j = j.wrapping_shr(8u32) | (block[i as usize] as u16).wrapping_shl(8u32) as i32;
        ftab[j as usize] = (ftab[j as usize]).wrapping_add(1u32)
      };
      i = i.wrapping_sub(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 2i32.wrapping_add(12i32).wrapping_add(18i32).wrapping_add(2i32)
    {
      {
        block[nblock.wrapping_add(i) as usize] = block[i as usize];
        quadrant[nblock.wrapping_add(i) as usize] = 0u16
      };
      i = i.wrapping_add(1i32)
    }
  };
  if verb >= 4i32 { () };
  {
    i = 1i32;
    while
    i <= 65536i32
    {
      ftab[i as usize] = (ftab[i as usize]).wrapping_add(ftab[i.wrapping_sub(1i32) as usize]);
      i = i.wrapping_add(1i32)
    }
  };
  s = (block[0usize]).wrapping_shl(8u32) as u16;
  i = nblock.wrapping_sub(1i32);
  {
    while
    i >= 3i32
    {
      {
        s = s.wrapping_shr(8u32) | (block[i as usize]).wrapping_shl(8u32) as u16;
        j = (ftab[s as usize]).wrapping_sub(1u32) as i32;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i as u32;
        s = s.wrapping_shr(8u32) | (block[i.wrapping_sub(1i32) as usize]).wrapping_shl(8u32) as u16;
        j = (ftab[s as usize]).wrapping_sub(1u32) as i32;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i.wrapping_sub(1i32) as u32;
        s = s.wrapping_shr(8u32) | (block[i.wrapping_sub(2i32) as usize]).wrapping_shl(8u32) as u16;
        j = (ftab[s as usize]).wrapping_sub(1u32) as i32;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i.wrapping_sub(2i32) as u32;
        s = s.wrapping_shr(8u32) | (block[i.wrapping_sub(3i32) as usize]).wrapping_shl(8u32) as u16;
        j = (ftab[s as usize]).wrapping_sub(1u32) as i32;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i.wrapping_sub(3i32) as u32
      };
      i = i.wrapping_sub(4i32)
    }
  };
  {
    while
    i >= 0i32
    {
      {
        s = s.wrapping_shr(8u32) | (block[i as usize]).wrapping_shl(8u32) as u16;
        j = (ftab[s as usize]).wrapping_sub(1u32) as i32;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i as u32
      };
      i = i.wrapping_sub(1i32)
    }
  };
  {
    i = 0i32;
    while
    i <= 255i32
    {
      {
        bigDone[i as usize] = 0i32 as u8;
        runningOrder[i as usize] = i
      };
      i = i.wrapping_add(1i32)
    }
  };
  {
    let mut vv: i32;
    let mut h: i32 = 1i32;
    {
      h = 3i32.wrapping_mul(h).wrapping_add(1i32);
      while h <= 256i32 { h = 3i32.wrapping_mul(h).wrapping_add(1i32) }
    };
    {
      h = h.wrapping_div(3i32);
      i = h;
      while
      i <= 255i32
      {
        {
          vv = runningOrder[i as usize];
          j = i;
          while
          (ftab[(runningOrder[j.wrapping_sub(h) as usize]).wrapping_add(1i32).wrapping_shl(8u32)
          as
          usize]).wrapping_sub(
            ftab[(runningOrder[j.wrapping_sub(h) as usize]).wrapping_shl(8u32) as usize]
          )
          >
          (ftab[vv.wrapping_add(1i32).wrapping_shl(8u32) as usize]).wrapping_sub(
            ftab[vv.wrapping_shl(8u32) as usize]
          )
          {
            runningOrder[j as usize] = runningOrder[j.wrapping_sub(h) as usize];
            j = j.wrapping_sub(h);
            if j <= h.wrapping_sub(1i32) { break }
          };
          runningOrder[j as usize] = vv
        };
        i = i.wrapping_add(1i32)
      }
    };
    while
    h != 1i32
    {
      h = h.wrapping_div(3i32);
      i = h;
      while
      i <= 255i32
      {
        {
          vv = runningOrder[i as usize];
          j = i;
          while
          (ftab[(runningOrder[j.wrapping_sub(h) as usize]).wrapping_add(1i32).wrapping_shl(8u32)
          as
          usize]).wrapping_sub(
            ftab[(runningOrder[j.wrapping_sub(h) as usize]).wrapping_shl(8u32) as usize]
          )
          >
          (ftab[vv.wrapping_add(1i32).wrapping_shl(8u32) as usize]).wrapping_sub(
            ftab[vv.wrapping_shl(8u32) as usize]
          )
          {
            runningOrder[j as usize] = runningOrder[j.wrapping_sub(h) as usize];
            j = j.wrapping_sub(h);
            if j <= h.wrapping_sub(1i32) { break }
          };
          runningOrder[j as usize] = vv
        };
        i = i.wrapping_add(1i32)
      }
    }
  };
  numQSorted = 0i32;
  {
    i = 0i32;
    while
    i <= 255i32
    {
      {
        ss = runningOrder[i as usize];
        {
          j = 0i32;
          while
          j <= 255i32
          {
            if j != ss
            {
              sb = ss.wrapping_shl(8u32).wrapping_add(j);
              if ftab[sb as usize] & 1i32.wrapping_shl(21u32) as u32 == 0u32
              {
                let lo: i32 = (ftab[sb as usize] & ! 1i32.wrapping_shl(21u32) as u32) as i32;
                let hi: i32 =
                    (ftab[sb.wrapping_add(1i32) as usize] & ! 1i32.wrapping_shl(21u32) as u32).wrapping_sub(
                      1u32
                    )
                    as
                    i32;
                if hi > lo
                {
                  if verb >= 4i32 { () };
                  mainQSort3(ptr, block, quadrant, nblock, lo, hi, 2i32, budget);
                  numQSorted = numQSorted.wrapping_add(hi.wrapping_sub(lo).wrapping_add(1i32));
                  if budget[0usize] < 0i32 { return () }
                }
              };
              ftab[sb as usize] |= 1i32.wrapping_shl(21u32) as u32
            };
            j = j.wrapping_add(1i32)
          }
        };
        {
          {
            j = 0i32;
            while
            j <= 255i32
            {
              {
                copyStart[j as usize] =
                    (ftab[j.wrapping_shl(8u32).wrapping_add(ss) as usize]
                    &
                    ! 1i32.wrapping_shl(21u32) as u32)
                    as
                    i32;
                copyEnd[j as usize] =
                    (ftab[j.wrapping_shl(8u32).wrapping_add(ss).wrapping_add(1i32) as usize]
                    &
                    ! 1i32.wrapping_shl(21u32) as u32).wrapping_sub(1u32)
                    as
                    i32
              };
              j = j.wrapping_add(1i32)
            }
          };
          {
            j = (ftab[ss.wrapping_shl(8u32) as usize] & ! 1i32.wrapping_shl(21u32) as u32) as i32;
            while
            j < copyStart[ss as usize]
            {
              {
                k = (ptr[j as usize]).wrapping_sub(1u32) as i32;
                if k < 0i32 { k = k.wrapping_add(nblock) };
                c1 = block[k as usize];
                if bigDone[c1 as usize] == 0u8
                {
                  ptr[(copyStart[c1 as usize] = (copyStart[c1 as usize]).wrapping_add(1i32))
                  as
                  usize] =
                      k as u32
                }
              };
              j = j.wrapping_add(1i32)
            }
          };
          j =
              (ftab[ss.wrapping_add(1i32).wrapping_shl(8u32) as usize]
              &
              ! 1i32.wrapping_shl(21u32) as u32).wrapping_sub(1u32)
              as
              i32;
          while
          j > copyEnd[ss as usize]
          {
            {
              k = (ptr[j as usize]).wrapping_sub(1u32) as i32;
              if k < 0i32 { k = k.wrapping_add(nblock) };
              c1 = block[k as usize];
              if bigDone[c1 as usize] == 0u8
              {
                ptr[(copyEnd[c1 as usize] = (copyEnd[c1 as usize]).wrapping_sub(1i32)) as usize] =
                    k as u32
              }
            };
            j = j.wrapping_sub(1i32)
          }
        };
        {
          j = 0i32;
          while
          j <= 255i32
          {
            ftab[j.wrapping_shl(8u32).wrapping_add(ss) as usize] |= 1i32.wrapping_shl(21u32) as u32;
            j = j.wrapping_add(1i32)
          }
        };
        bigDone[ss as usize] = 1i32 as u8;
        if i < 255i32
        {
          let bbStart: i32 =
              (ftab[ss.wrapping_shl(8u32) as usize] & ! 1i32.wrapping_shl(21u32) as u32) as i32;
          let bbSize: i32 =
              (ftab[ss.wrapping_add(1i32).wrapping_shl(8u32) as usize]
              &
              ! 1i32.wrapping_shl(21u32) as u32).wrapping_sub(bbStart as u32)
              as
              i32;
          let mut shifts: i32 = 0i32;
          while bbSize.wrapping_shr(shifts as u32) > 65534i32 { shifts = shifts.wrapping_add(1i32) };
          j = bbSize.wrapping_sub(1i32);
          while
          j >= 0i32
          {
            {
              let a2update: i32 = ptr[bbStart.wrapping_add(j) as usize] as i32;
              let qVal: u16 = j.wrapping_shr(shifts as u32) as u16;
              quadrant[a2update as usize] = qVal;
              if a2update < 2i32.wrapping_add(12i32).wrapping_add(18i32).wrapping_add(2i32)
              { quadrant[a2update.wrapping_add(nblock) as usize] = qVal }
            };
            j = j.wrapping_sub(1i32)
          }
        }
      };
      i = i.wrapping_add(1i32)
    }
  };
  if verb >= 4i32 { () }
}

#[inline] pub fn mmed3(mut a: u8, mut b: u8, c: u8) -> u8
{
  let mut t: u8;
  if a > b
  {
    t = a;
    a = b;
    b = t
  };
  if b > c
  {
    b = c;
    if a > b { b = a }
  };
  return b
}
