mod wrapper;
pub use wrapper::WrapperGen;

extern crate bitflags;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use bitflags::bitflags;

bitflags! {
    pub struct GenOpt: u32 {
        /// Generate implementation for trait `::protobuf::Message`.
        const MESSAGE = 0b0000_0001;
        /// Generate getters.
        const TRIVIAL_GET = 0b0000_0010;
        /// Generate setters.
        const TRIVIAL_SET = 0b0000_0100;
        /// Generate the `new_` constructors.
        const NEW = 0b0000_1000;
        /// Generate `clear_*` functions.
        const CLEAR = 0b0001_0000;
        /// Generate `has_*` functions.
        const HAS = 0b0010_0000;
        /// Generate mutable getters.
        const MUT = 0b0100_0000;
        /// Generate `take_*` functions.
        const TAKE = 0b1000_0000;
        /// Except `impl protobuf::Message`.
        const NO_MSG = Self::TRIVIAL_GET.bits
         | Self::TRIVIAL_SET.bits
         | Self::CLEAR.bits
         | Self::HAS.bits
         | Self::MUT.bits
         | Self::TAKE.bits;
        /// Except `new_` and `impl protobuf::Message`.
        const ACCESSOR = Self::TRIVIAL_GET.bits
         | Self::TRIVIAL_SET.bits
         | Self::MUT.bits
         | Self::TAKE.bits;
    }
}
