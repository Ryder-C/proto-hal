use proto_hal::macros::block;

#[block(base_addr = 0x4001_0400, auto_increment, erase_mod)]
mod exti {
    #[register(auto_increment)]
    mod imr1 {
        #[field(width = 1, read, write)]
        mod im0 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im1 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im2 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im3 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im4 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im5 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im6 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im7 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im8 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im9 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im10 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im11 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im12 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im13 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im14 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im15 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im16 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im17 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im18 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im19 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im20 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im21 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im22 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im23 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im24 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im25 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im26 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im27 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im28 {
            #[state(bits = 0)]
            struct Masked;

            #[state(bits = 1, reset)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im29 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im30 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod im31 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
    }

    #[register(auto_increment)]
    mod emr1 {
        #[field(width = 1, read, write)]
        mod em0 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em1 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em2 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em3 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em4 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em5 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em6 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em7 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em8 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em9 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em10 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em11 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em12 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em13 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em14 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em15 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em16 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em17 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em18 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em19 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em20 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em21 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em22 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em23 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em24 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em25 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em26 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em27 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em28 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em29 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em30 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
        #[field(width = 1, read, write)]
        mod em31 {
            #[state(bits = 0, reset)]
            struct Masked;

            #[state(bits = 1)]
            struct Unmasked;
        }
    }

    #[register(auto_increment)]
    mod rtsr1 {
        #[field(width = 1, read, write)]
        mod rt0 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt1 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt2 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt3 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt4 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt5 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt6 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt7 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt8 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt9 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt10 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt11 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt12 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt13 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt14 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt15 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt16 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt17 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(offset = 19, width = 1, read, write)]
        mod rt19 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt20 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt21 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt22 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(offset = 29, width = 1, read, write)]
        mod rt29 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt30 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod rt31 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
    }

    #[register(auto_increment)]
    mod ftsr1 {
        #[field(width = 1, read, write)]
        mod ft0 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft1 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft2 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft3 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft4 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft5 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft6 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft7 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft8 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft9 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft10 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft11 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft12 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft13 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft14 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft15 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft16 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft17 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(offset = 19, width = 1, read, write)]
        mod ft19 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft20 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft21 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft22 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(offset = 29, width = 1, read, write)]
        mod ft29 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft30 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
        #[field(width = 1, read, write)]
        mod ft31 {
            #[state(bits = 0, reset)]
            struct Disabled;

            #[state(bits = 1)]
            struct Enabled;
        }
    }

    // a little weird, RM doesn't
    // say anything about writing 0
    // to these fields...
    #[register(auto_increment)]
    mod swier1 {
        #[field(width = 1, read, write, reset = 0)]
        mod swi0 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi1 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi2 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi3 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi4 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi5 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi6 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi7 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi8 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi9 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi10 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi11 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi12 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi13 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi14 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi15 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi16 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi17 {}
        #[field(offset = 19, width = 1, read, write, reset = 0)]
        mod swi19 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi20 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi21 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi22 {}
        #[field(offset = 29, width = 1, read, write, reset = 0)]
        mod swi29 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi30 {}
        #[field(width = 1, read, write, reset = 0)]
        mod swi31 {}
    }

    // a little weird, RM doesn't
    // say anything about writing 0
    // to these fields...
    #[register(auto_increment)]
    mod pr1 {
        #[field(width = 1, read, write)]
        mod pif0 {}
        #[field(width = 1, read, write)]
        mod pif1 {}
        #[field(width = 1, read, write)]
        mod pif2 {}
        #[field(width = 1, read, write)]
        mod pif3 {}
        #[field(width = 1, read, write)]
        mod pif4 {}
        #[field(width = 1, read, write)]
        mod pif5 {}
        #[field(width = 1, read, write)]
        mod pif6 {}
        #[field(width = 1, read, write)]
        mod pif7 {}
        #[field(width = 1, read, write)]
        mod pif8 {}
        #[field(width = 1, read, write)]
        mod pif9 {}
        #[field(width = 1, read, write)]
        mod pif10 {}
        #[field(width = 1, read, write)]
        mod pif11 {}
        #[field(width = 1, read, write)]
        mod pif12 {}
        #[field(width = 1, read, write)]
        mod pif13 {}
        #[field(width = 1, read, write)]
        mod pif14 {}
        #[field(width = 1, read, write)]
        mod pif15 {}
        #[field(width = 1, read, write)]
        mod pif16 {}
        #[field(width = 1, read, write)]
        mod pif17 {}
        #[field(offset = 19, width = 1, read, write)]
        mod pif19 {}
        #[field(width = 1, read, write)]
        mod pif20 {}
        #[field(width = 1, read, write)]
        mod pif21 {}
        #[field(width = 1, read, write)]
        mod pif22 {}
        #[field(offset = 29, width = 1, read, write)]
        mod pif29 {}
        #[field(width = 1, read, write)]
        mod pif30 {}
        #[field(width = 1, read, write)]
        mod pif31 {}
    }

    // TODO...
}
