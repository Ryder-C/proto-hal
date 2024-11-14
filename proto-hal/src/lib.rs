#![no_std]

pub mod regs;

pub mod gpio;

#[cfg(feature = "stm32")]
pub mod stm32;

pub use macros;
pub mod macro_utils;

/// Types that encapsulate a resource that can be configured to be
/// in a "reset" state implement this trait.
pub trait IntoReset {
    /// The form of the implementor type in the "reset" state.
    type Reset;

    /// Transform the implementor type into the "reset" state.
    fn into_reset(self) -> Self::Reset;
}

#[cfg(test)]
mod tests {
    mod macros {
        use arbitrary_int::u4;
        use macros::field;

        // #[peripheral]
        mod syscfg {
            use super::*;

            #[field(width = 3, read, write(entitlements = [N0]))]
            enum MemMode {
                #[reset]
                MainFlash,
                #[state(entitlements = [N1])]
                SystemFlash,
                Fsmc,
                SRam1,
                QuadSpi,
            }

            // #[register(infer_offsets)]
            // struct MemRmp {
            //     mem_mode: MemMode,

            //     #[field(offset = 0x08)]
            //     fb_mode: bool,
            // }
            // struct Cfgr1;
            // struct ExtiCr1;
            // struct ExtiCr2;

            // #[block(base_addr = 0x4001_0000, infer_offsets)]
            // struct SysCfg {
            //     memrmp: MemRmp,
            //     cfgr1: Cfgr1,
            //     exticr1: ExtiCr1,
            //     exticr2: ExtiCr2,
            // }
        }

        // #[peripheral]
        // pub mod cordic {
        //     #[field(width = 4, read, write)]
        //     pub enum Func {
        //         #[reset]
        //         Cos,
        //         Sin,
        //         Angle,
        //         Magnitude,
        //         Arctangent,
        //         HyperbolicCosine,
        //         HyperbolicSine,
        //         ArctanH,
        //         Ln,
        //         Sqrt,
        //     }

        //     #[register(infer_offsets)]
        //     pub struct Csr {
        //         func: Func,
        //         precision: u4,
        //         scale: u3,
        //         #[field(offset = 0x10)]
        //         ien: bool,
        //         dmaren: bool,
        //         dmawen: bool,
        //         nres: bool,
        //         nargs: bool,
        //         ressize: bool,
        //         argsize: bool,
        //         #[field(offset = 0x1f)]
        //         rrdy: bool,
        //     }

        //     #[register(infer_offsets)]
        //     pub struct WData {
        //         arg: u32,
        //     }

        //     #[register(infer_offsets)]
        //     pub struct RData {
        //         res: u32,
        //     }

        //     #[block(base_addr = 0x4002_1000, infer_offsets)]
        //     pub struct Cordic {
        //         csr: Csr,
        //         wdata: WData,
        //         rdata: RData,
        //     }
        // }
    }
}
