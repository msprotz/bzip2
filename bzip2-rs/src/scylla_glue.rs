// HAND-WRITTEN FILE

pub(crate)
fn scylla_u8_of_u32(x: & mut [u32]) -> & mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u8>(), x.len()*4)
    }
}
