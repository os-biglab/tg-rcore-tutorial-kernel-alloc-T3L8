//! 内存分配。
//!
//! 教程阅读建议：
//!
//! - 先看 `init` 与 `transfer`：理解“先初始化，再把可用内存交给分配器”；
//! - 再看 `HEAP` / `GlobalAlloc`：理解 Rust `alloc` 如何落到内核堆实现。

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use core::alloc::GlobalAlloc;
use buddy_system_allocator::LockedHeap;

/// 堆分配器。
///
/// 使用了自带锁的 buddy_system_allocator::LockedHeap
#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

/// 初始化内存分配。
///
/// 参数 `base_address` 表示动态内存区域的起始位置。
///
/// # 注意
///
/// 此函数必须在使用任何堆分配之前调用，且只能调用一次。
#[inline]
pub fn init(_base_address: usize) {
    // nothing to do for LockedHeap until transfer
}

/// 将一个内存块托管到内存分配器。
///
/// # Safety
///
/// 调用者必须确保：
/// - `region` 内存块与已经转移到分配器的内存块都不重叠
/// - `region` 未被其他对象引用
/// - `region` 必须位于初始化时传入的起始位置之后
/// - 内存块的所有权将转移到分配器
#[inline]
pub unsafe fn transfer(region: &'static mut [u8]) {
    // 将一段“现成内存”并入堆。常用于把启动后可回收区域纳入分配器管理。
    // SAFETY: 由调用者保证内存块有效且不重叠
    HEAP.lock().init(region.as_mut_ptr() as usize, region.len());
}
