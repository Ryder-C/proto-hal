use proto_hal::macros::block;

#[block(base_addr = 0x4002_1000, auto_increment, erase_mod)]
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

    #[register(auto_increment)]
    mod ahb2enr {
        #[field(width = 1, read, write)]
        mod gpioaen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpioben {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpiocen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpioden {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpioeen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpiofen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod gpiogen {
            #[state(bits = 0, reset)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
    }
}
