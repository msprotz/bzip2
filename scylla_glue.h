#include "bzlib_private.h"

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline UChar *scylla_u8_of_u32(UInt32 *src) {
  return (UChar *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline UInt16 *scylla_u16_of_u8(UChar *src) {
  return (UInt16 *)src;
}

// SCYLLA: this breaks strict aliasing, which the original bzlib2 code did do anyhow.
__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline UInt16 *scylla_u16_of_u32(UInt32 *src) {
  return (UInt16 *)src;
}

// we rewrite s->zbits[i] = v
#define ZBITS_WRITE(s, i, v) UChar *zbits = scylla_u8_of_u32((s)->arr2) + (s)->zbits_ofs; zbits[i] = v
#define ZBITS_READ(s, i) (scylla_u8_of_u32((s)->arr2) + (s)->zbits_ofs)[i]

#define MTFV(s) (scylla_u16_of_u32(s->arr1))
