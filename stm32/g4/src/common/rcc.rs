use proto_hal::macros::block;

#[block(base_addr = 0x4002_1000, auto_increment, erase_mod)]
mod rcc {
    #[register(offset = 0x48)]
    mod ahb1enr {
        #[field(offset = 3, width = 1, read, write, reset = Disabled)]
        mod cordicen {
            #[state(bits = 0)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
    }

    #[register(auto_increment)]
    mod ahb2enr {
        #[schema(width = 1)]
        mod enable {
            #[state(bits = 0)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }

        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpioaen {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpioben {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpiocen {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpioden {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpioeen {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpiofen {}
        #[field(width = 1, schema = enable, read, write, reset = Disabled)]
        mod gpiogen {}
    }

    #[register(offset = 0x60)]
    mod apb2enr {
        #[field(offset = 0, width = 1, read, write, reset = Disabled)]
        mod syscfgen {
            #[state(bits = 0)]
            struct Disabled;
            #[state(bits = 1)]
            struct Enabled;
        }
    }
}
