//! Allocation and container census (`--features stats`): thread-local counters bumped by the hot
//! constructors of List/Map/Str, printed by [`PrintOnDrop`] when the program's PHP thread ends
//! (including through a PHP `exit()`, which unwinds). Without the feature every hook is a no-op.

#[cfg(feature = "stats")]
use std::cell::Cell;

pub const LIST_NEW_EMPTY: usize = 0;
pub const LIST_WITH_CAP: usize = 1;
pub const LIST_FROM_VEC_0: usize = 2;
pub const LIST_FROM_VEC_1: usize = 3;
pub const LIST_FROM_VEC_2: usize = 4;
pub const LIST_FROM_VEC_3_4: usize = 5;
pub const LIST_FROM_VEC_5_8: usize = 6;
pub const LIST_FROM_VEC_MORE: usize = 7;
pub const LIST_COW_CLONE: usize = 8;
pub const LIST_GROW: usize = 9;
pub const LIST_DROP_LAST: usize = 10;
pub const LIST_DROP_LAST_EMPTY: usize = 11;
pub const MAP_ALLOC: usize = 12;
pub const MAP_COW_CLONE: usize = 13;
pub const MAP_FIND: usize = 14;
pub const MAP_INSERT: usize = 15;
pub const MAP_DROP_LAST: usize = 16;
pub const STR_HEAP_ALLOC: usize = 17;
pub const STR_HEAP_ALLOC_LE_32: usize = 18;
pub const STR_HEAP_ALLOC_LE_128: usize = 19;
pub const STR_HASH_COMPUTE: usize = 20;
pub const STR_LOWER: usize = 21;
pub const STR_CONCAT: usize = 22;
pub const STR_COW_GROW: usize = 23;
pub const OBJ_NEW: usize = 24;
pub const MAP_REACH_1: usize = 25;
pub const MAP_REACH_2: usize = 26;
pub const MAP_REACH_3: usize = 27;
pub const MAP_REACH_5: usize = 28;
pub const MAP_REACH_9: usize = 29;
pub const MAP_REACH_17: usize = 30;
pub const MAP_REACH_65: usize = 31;
pub const PROP_GET_CLONE: usize = 32;
pub const STR_HEAP_ALLOC_LE_23: usize = 33;
const N: usize = 34;

const NAMES: [&str; N] = [
    "List::new (empty)", "List::with_capacity", "List::from_vec len=0", "List::from_vec len=1", "List::from_vec len=2",
    "List::from_vec len=3-4", "List::from_vec len=5-8", "List::from_vec len>8", "List COW clone (shared write)",
    "List buffer grow (push at capacity)", "List last-ref drop", "List last-ref drop (empty)",
    "Map first allocation", "Map COW clone (shared write)", "Map find", "Map insert", "Map last-ref drop",
    "Str heap alloc", "Str heap alloc <=32B", "Str heap alloc <=128B", "Str hash computed", "Str to_lowercase",
    "Str concat", "Str COW grow (shared push)", "object new",
    "Map reached 1 entry", "Map reached 2 entries", "Map reached 3", "Map reached 5", "Map reached 9", "Map reached 17", "Map reached 65",
    "property read cloning a non-Copy value (Rc/Str/List/Map)", "Str heap alloc <=23B (would fit a 24-byte inline Str)",
];

#[cfg(feature = "stats")]
thread_local! {
    static COUNTERS: [Cell<u64>; N] = [const { Cell::new(0) }; N];
}

#[inline(always)]
pub fn bump(i: usize) {
    #[cfg(feature = "stats")]
    COUNTERS.with(|c| c[i].set(c[i].get() + 1));
    #[cfg(not(feature = "stats"))]
    let _ = i;
}

pub fn print() {
    #[cfg(feature = "stats")]
    COUNTERS.with(|c| {
        eprintln!("=== php-rt census (this thread) ===");
        for (i, name) in NAMES.iter().enumerate() {
            eprintln!("{:>14}  {}", c[i].get(), name);
        }
    });
}

/// Prints the census when dropped (also while unwinding out of a PHP `exit()`).
pub struct PrintOnDrop;

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        print();
    }
}
