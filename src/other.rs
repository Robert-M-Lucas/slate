use core::hint::black_box;

#[deprecated]
pub fn arbitrary_delay() {
    for x in 0..5_000_000 {
        black_box(x);
    }
}
