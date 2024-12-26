use proto_hal::macros::block;

#[block(
    base_addr = 0x4002_0c00,
    entitlements = [super::rcc::ahb1enr::cordicen::Enabled],
    auto_increment,
    erase_mod,
)]
mod cordic {
    #[register(read, auto_increment)]
    mod csr {
        #[field(width = 4, write, reset = Cos, auto_increment)]
        mod func {
            #[state(entitlements = [scale::N0])]
            struct Cos;

            #[state(entitlements = [scale::N0])]
            struct Sin;

            #[state(entitlements = [scale::N0])]
            struct ATan2;

            #[state(entitlements = [scale::N0])]
            struct Magnitude;

            #[state]
            struct ATan;

            #[state(entitlements = [scale::N1])]
            struct CosH;

            #[state(entitlements = [scale::N1])]
            struct SinH;

            #[state(entitlements = [scale::N1])]
            struct ATanH;

            #[state(entitlements = [scale::N1, scale::N2, scale::N3, scale::N4])]
            struct Ln;

            #[state(entitlements = [scale::N0, scale::N1, scale::N2])]
            struct Sqrt;
        }

        #[field(width = 4, write, reset = P20, auto_increment)]
        /// custom docs
        mod precision {
            #[state(bits = 1)]
            struct P4;
            #[state]
            struct P8;
            #[state]
            struct P12;
            #[state]
            struct P16;
            #[state]
            struct P20;
            #[state]
            struct P24;
            #[state]
            struct P28;
            #[state]
            struct P32;
            #[state]
            struct P36;
            #[state]
            struct P40;
            #[state]
            struct P44;
            #[state]
            struct P48;
            #[state]
            struct P52;
            #[state]
            struct P56;
            #[state]
            struct P60;
        }

        #[field(width = 3, write, reset = N0, auto_increment)]
        mod scale {
            #[state]
            struct N0;
            #[state]
            struct N1;
            #[state]
            struct N2;
            #[state]
            struct N3;
            #[state]
            struct N4;
            #[state]
            struct N5;
            #[state]
            struct N6;
            #[state]
            struct N7;
        }

        #[schema(width = 1)]
        mod enable {
            #[state(bits = 0)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }

        #[field(offset = 16, schema = enable, write, reset = Disabled)]
        mod ien {}

        #[field(schema = enable, write, reset = Disabled)]
        mod dmaren {}

        #[field(schema = enable, write, reset = Disabled)]
        mod dmawen {}

        #[field(width = 1, write, reset = OneRead)]
        mod nres {
            #[state(bits = 0)]
            struct OneRead;
            #[state(bits = 1, entitlements = [ressize::Q31])]
            struct TwoReads;
        }

        #[field(width = 1, write, reset = OneWrite)]
        mod nargs {
            #[state(bits = 0)]
            struct OneWrite;
            #[state(bits = 1, entitlements = [argsize::Q31])]
            struct TwoWrites;
        }

        #[schema(width = 1)]
        mod data {
            #[state(bits = 0)]
            struct Q31;
            #[state(bits = 1)]
            struct Q15;
        }

        #[field(schema = data, write, reset = Q31)]
        mod ressize {}

        #[field(schema = data, write, reset = Q31)]
        mod argsize {}

        #[field(offset = 31, width = 1, reset = NoData)]
        mod rrdy {
            #[state(bits = 0)]
            struct NoData;
            #[state(bits = 1)]
            struct DataReady;
        }
    }

    #[register(width = 32, write(effect = unresolve(csr::rrdy)))]
    mod wdata {
        #[field(offset = 0)]
        mod arg {}
    }

    #[register(width = 32, read(entitlements = [csr::rrdy::Ready], effect = unresolve(csr::rrdy)), reset = 0)]
    mod rdata {
        #[field(offset = 0)]
        mod res {}
    }
}
