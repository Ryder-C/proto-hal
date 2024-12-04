use proto_hal::macros::block;

#[block(base_addr = 0x4002_1000, erase_mod)]
mod rcc {
    #[register(offset = 0x48)]
    mod ahb1enr {
        #[field(offset = 3, width = 1, read, write)]
        mod cordicen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
    }
}
