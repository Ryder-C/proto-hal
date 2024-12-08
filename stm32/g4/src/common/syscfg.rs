use proto_hal::macros::block;

#[block(base_addr = 0x4001_0000, entitlements = [super::rcc::apb2enr::syscfgen::Enabled], erase_mod)]
mod syscfg {
    #[register(offset = 0x14, auto_increment)]
    mod exticr4 {
        #[field(width = 4, read, write, auto_increment)]
        mod exti12 {
            #[state(reset)]
            struct PA;
            #[state]
            struct PB;
            #[state]
            struct PC;
            #[state]
            struct PD;
            #[state]
            struct PE;
            #[state]
            struct PF;
        }
        #[field(width = 4, read, write, auto_increment)]
        mod exti13 {
            #[state(reset)]
            struct PA;
            #[state]
            struct PB;
            #[state]
            struct PC;
            #[state]
            struct PD;
            #[state]
            struct PE;
            #[state]
            struct PF;
        }
        #[field(width = 4, read, write, auto_increment)]
        mod exti14 {
            #[state(reset)]
            struct PA;
            #[state]
            struct PB;
            #[state]
            struct PC;
            #[state]
            struct PD;
            #[state]
            struct PE;
            #[state]
            struct PF;
        }
        #[field(width = 4, read, write, auto_increment)]
        mod exti15 {
            #[state(reset)]
            struct PA;
            #[state]
            struct PB;
            #[state]
            struct PC;
            #[state]
            struct PD;
            #[state]
            struct PE;
            #[state]
            struct PF;
        }
    }
}
