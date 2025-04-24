#include "bzlib_private.h"

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability(mut)")))
static inline UChar *scylla_u8_of_u32(UInt32 *src) {
  return (UChar *)src;
}
