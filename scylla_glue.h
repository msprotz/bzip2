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
