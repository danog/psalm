# axis-7 php-rt reference (Arc + RwLock + atomic Str, Send+Sync)

Validated copies of the php-rt files converted for axis-7 (multithread-ready storage),
proven on the tiny suite (compiles + 20/20 PASS + the driver's `_assert_send_sync`
guard confirms Str/Map/List/concrete-handles are Send+Sync).

The canonical php-rt on ct is ahead of the transpiler repo's `rust/php-rt` (a pre-existing
sync gap), and `string.rs` there has a different Str layout, so these can't be blindly
diffed onto `rust/php-rt`. Apply this recipe to ct's php-rt on reconciliation:

1. `Rc -> Arc`: `pub use std::sync::Arc as Rc` in the prelude + per-file `use std::rc::Rc`
   -> `use std::sync::Arc as Rc`; fully-qualified `std::rc::Rc` -> `std::sync::Arc`.
2. `RefCell -> RwCell`: add `parking_lot = "0.12"`; define `RwCell` (support.rs) wrapping
   `parking_lot::RwLock` with `.borrow()/.borrow_mut()` via `try_read()/try_write()` returning
   MAPPED guards (`CellRef`/`CellRefMut`); prelude aliases `RwCell as RefCell, CellRef as Ref,
   CellRefMut as RefMut`; `PropRef`/`PropMut` use the mapped guards. Thread_local RefCells stay std::cell.
3. `Str` atomic (string.rs): `Hdr` `Cell<usize/u64>` -> `AtomicUsize/AtomicU64`; clone =
   `strong.fetch_add(1, Relaxed)`; drop = `strong.fetch_sub(1, Release)==1 -> fence(Acquire)+dealloc`;
   get/set -> load/store(Relaxed); `unsafe impl Send/Sync for HeapStr`.

STILL TODO for full Send+Sync: bound `dyn PhpObject` / `dyn Fn` (Mixed's closures) + Send + Sync
(ripples to impls), then thread scan/analyze (std::thread::scope) -- needs ct for the speedup.
