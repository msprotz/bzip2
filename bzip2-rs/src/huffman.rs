#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]

pub fn BZ2_hbAssignCodes(
  code: &mut [i32],
  length: &[u8],
  minLen: i32,
  maxLen: i32,
  alphaSize: i32
)
{
  let mut n: i32;
  let mut vec: i32;
  let mut i: i32;
  vec = 0i32;
  n = minLen;
  while
  n <= maxLen
  {
    {
      {
        i = 0i32;
        while
        i < alphaSize
        {
          if length[i as usize] == n as u8
          {
            code[i as usize] = vec;
            vec = vec.wrapping_add(1i32)
          };
          i = i.wrapping_add(1i32)
        }
      };
      vec = vec.wrapping_shl(1u32)
    };
    n = n.wrapping_add(1i32)
  }
}

pub fn BZ2_hbCreateDecodeTables(
  limit: &mut [i32],
  base: &mut [i32],
  perm: &mut [i32],
  length: &[u8],
  minLen: i32,
  maxLen: i32,
  alphaSize: i32
)
{
  let mut pp: i32;
  let mut i: i32;
  let mut j: i32;
  let mut vec: i32;
  pp = 0i32;
  {
    i = minLen;
    while
    i <= maxLen
    {
      {
        j = 0i32;
        while
        j < alphaSize
        {
          if length[j as usize] == i as u8
          {
            perm[pp as usize] = j;
            pp = pp.wrapping_add(1i32)
          };
          j = j.wrapping_add(1i32)
        }
      };
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 23i32
    {
      base[i as usize] = 0i32;
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < alphaSize
    {
      base[(length[i as usize]).wrapping_add(1u8) as usize] =
          (base[(length[i as usize]).wrapping_add(1u8) as usize]).wrapping_add(1i32);
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 1i32;
    while
    i < 23i32
    {
      base[i as usize] = (base[i as usize]).wrapping_add(base[i.wrapping_sub(1i32) as usize]);
      i = i.wrapping_add(1i32)
    }
  };
  {
    i = 0i32;
    while
    i < 23i32
    {
      limit[i as usize] = 0i32;
      i = i.wrapping_add(1i32)
    }
  };
  vec = 0i32;
  {
    i = minLen;
    while
    i <= maxLen
    {
      {
        vec = vec.wrapping_add((base[i.wrapping_add(1i32) as usize]).wrapping_sub(base[i as usize]));
        limit[i as usize] = vec.wrapping_sub(1i32);
        vec = vec.wrapping_shl(1u32)
      };
      i = i.wrapping_add(1i32)
    }
  };
  i = minLen.wrapping_add(1i32);
  while
  i <= maxLen
  {
    base[i as usize] =
        (limit[i.wrapping_sub(1i32) as usize]).wrapping_add(1i32).wrapping_shl(1u32).wrapping_sub(
          base[i as usize]
        );
    i = i.wrapping_add(1i32)
  }
}

pub fn BZ2_hbMakeCodeLengths(len: &mut [u8], freq: &[i32], alphaSize: i32, maxLen: i32)
{
  let mut nNodes: i32;
  let mut nHeap: i32;
  let mut n1: i32;
  let mut n2: i32;
  let mut i: i32;
  let mut j: i32;
  let mut k: i32;
  let mut tooLong: u8;
  let mut heap: [i32; 260] = Default::default();
  let mut weight: [i32; 516] = Default::default();
  let mut parent: [i32; 516] = Default::default();
  {
    i = 0i32;
    while
    i < alphaSize
    {
      weight[i.wrapping_add(1i32) as usize] =
          if freq[i as usize] == 0i32 { 1i32 } else { freq[i as usize] }.wrapping_shl(8u32);
      i = i.wrapping_add(1i32)
    }
  };
  while
  1i32 as u8 != 0u8
  {
    nNodes = alphaSize;
    nHeap = 0i32;
    heap[0usize] = 0i32;
    weight[0usize] = 0i32;
    parent[0usize] = 0i32.wrapping_sub(2i32);
    {
      i = 1i32;
      while
      i <= alphaSize
      {
        {
          parent[i as usize] = 0i32.wrapping_sub(1i32);
          nHeap = nHeap.wrapping_add(1i32);
          heap[nHeap as usize] = i;
          let mut zz: i32;
          let mut tmp: i32;
          zz = nHeap;
          tmp = heap[zz as usize];
          while
          weight[tmp as usize] < weight[heap[zz.wrapping_shr(1u32) as usize] as usize]
          {
            heap[zz as usize] = heap[zz.wrapping_shr(1u32) as usize];
            zz = zz.wrapping_shr(1u32)
          };
          heap[zz as usize] = tmp
        };
        i = i.wrapping_add(1i32)
      }
    };
    while
    nHeap > 1i32
    {
      n1 = heap[1usize];
      heap[1usize] = heap[nHeap as usize];
      nHeap = nHeap.wrapping_sub(1i32);
      {
        let mut zz: i32;
        let mut yy: i32;
        let mut tmp: i32;
        zz = 1i32;
        tmp = heap[zz as usize];
        while
        1i32 as u8 != 0u8
        {
          yy = zz.wrapping_shl(1u32);
          if yy > nHeap { break };
          if
          yy < nHeap
          &&
          weight[heap[yy.wrapping_add(1i32) as usize] as usize] < weight[heap[yy as usize] as usize]
          { yy = yy.wrapping_add(1i32) };
          if weight[tmp as usize] < weight[heap[yy as usize] as usize] { break };
          heap[zz as usize] = heap[yy as usize];
          zz = yy
        };
        heap[zz as usize] = tmp
      };
      n2 = heap[1usize];
      heap[1usize] = heap[nHeap as usize];
      nHeap = nHeap.wrapping_sub(1i32);
      {
        let mut zz: i32;
        let mut yy: i32;
        let mut tmp: i32;
        zz = 1i32;
        tmp = heap[zz as usize];
        while
        1i32 as u8 != 0u8
        {
          yy = zz.wrapping_shl(1u32);
          if yy > nHeap { break };
          if
          yy < nHeap
          &&
          weight[heap[yy.wrapping_add(1i32) as usize] as usize] < weight[heap[yy as usize] as usize]
          { yy = yy.wrapping_add(1i32) };
          if weight[tmp as usize] < weight[heap[yy as usize] as usize] { break };
          heap[zz as usize] = heap[yy as usize];
          zz = yy
        };
        heap[zz as usize] = tmp
      };
      nNodes = nNodes.wrapping_add(1i32);
      parent[n1 as usize] = (parent[n2 as usize] = nNodes) as i32;
      weight[nNodes as usize] =
          (weight[n1 as usize] & 4294967040i32).wrapping_add(weight[n2 as usize] & 4294967040i32)
          |
          1i32.wrapping_add(
            if weight[n1 as usize] & 255i32 > weight[n2 as usize] & 255i32
            { weight[n1 as usize] & 255i32 }
            else
            { weight[n2 as usize] & 255i32 }
          );
      parent[nNodes as usize] = 0i32.wrapping_sub(1i32);
      nHeap = nHeap.wrapping_add(1i32);
      heap[nHeap as usize] = nNodes;
      let mut zz: i32;
      let mut tmp: i32;
      zz = nHeap;
      tmp = heap[zz as usize];
      while
      weight[tmp as usize] < weight[heap[zz.wrapping_shr(1u32) as usize] as usize]
      {
        heap[zz as usize] = heap[zz.wrapping_shr(1u32) as usize];
        zz = zz.wrapping_shr(1u32)
      };
      heap[zz as usize] = tmp
    };
    tooLong = 0i32 as u8;
    {
      i = 1i32;
      while
      i <= alphaSize
      {
        {
          j = 0i32;
          k = i;
          while
          parent[k as usize] >= 0i32
          {
            k = parent[k as usize];
            j = j.wrapping_add(1i32)
          };
          len[i.wrapping_sub(1i32) as usize] = j as u8;
          if j > maxLen { tooLong = 1i32 as u8 }
        };
        i = i.wrapping_add(1i32)
      }
    };
    if tooLong == 0u8 { break };
    i = 1i32;
    while
    i <= alphaSize
    {
      {
        j = (weight[i as usize]).wrapping_shr(8u32);
        j = 1i32.wrapping_add(j.wrapping_div(2i32));
        weight[i as usize] = j.wrapping_shl(8u32)
      };
      i = i.wrapping_add(1i32)
    }
  }
}
