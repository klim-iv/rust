//! A set of traits that define a stable interface to rustc's internals.
//!
//! These traits abstract rustc's internal APIs, allowing StableMIR to maintain a stable
//! interface regardless of internal compiler changes.

use rustc_middle::mir::interpret::AllocRange;
use rustc_middle::ty;
use rustc_middle::ty::Ty;
use rustc_span::def_id::DefId;

pub trait SmirTy<'tcx> {
    fn new_foreign(&self, def_id: DefId) -> Ty<'tcx>;
}

pub trait SmirTypingEnv<'tcx> {
    fn fully_monomorphized(&self) -> ty::TypingEnv<'tcx>;
}

pub trait SmirAllocRange<'tcx> {
    fn alloc_range(&self, offset: rustc_abi::Size, size: rustc_abi::Size) -> AllocRange;
}
