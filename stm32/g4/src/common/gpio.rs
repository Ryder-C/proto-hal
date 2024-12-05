use proto_hal::macros::block;

#[block(base_addr = 0x4800_0000, auto_increment)]
mod gpioa {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state(reset)]
            struct Alternate;
            #[state]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state(reset)]
            struct Alternate;
            #[state]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state(reset)]
            struct Alternate;
            #[state]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state(reset)]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state]
            struct None;
            #[state(reset)]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state]
            struct None;
            #[state]
            struct PullUp;
            #[state(reset)]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state]
            struct None;
            #[state(reset)]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }

    #[register(auto_increment)]
    mod idr {
        #[field(width = 1, read)]
        mod id0 {}
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpiob {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state(reset)]
            struct Alternate;
            #[state]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state(reset)]
            struct Alternate;
            #[state]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state]
            struct None;
            #[state(reset)]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpioc {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpiod {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpioe {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpiof {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}

#[block(base_addr = 0x4800_0400, auto_increment)]
mod gpiog {
    #[register(auto_increment)]
    mod moder {
        #[field(width = 2, read, write, auto_increment)]
        mod mode0 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode1 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode2 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode3 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode4 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode5 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode6 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode7 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode8 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode9 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode10 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode11 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode12 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode13 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode14 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod mode15 {
            #[state]
            struct Input;
            #[state]
            struct Output;
            #[state]
            struct Alternate;
            #[state(reset)]
            struct Analog;
        }
    }

    #[register(auto_increment)]
    mod otyper {
        #[field(width = 1, read, write, auto_increment)]
        mod ot0 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot1 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot2 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot3 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot4 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot5 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot6 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot7 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot8 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot9 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot10 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot11 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot12 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot13 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot14 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
        #[field(width = 1, read, write, auto_increment)]
        mod ot15 {
            #[state(reset)]
            struct PushPull;
            #[state]
            struct OpenDrain;
        }
    }

    #[register(auto_increment)]
    mod ospeedr {
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed0 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed1 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed2 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed3 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed4 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed5 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed6 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed7 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed8 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed9 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed10 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed11 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed12 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed13 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed14 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod ospeed15 {
            #[state(reset)]
            struct Low;
            #[state]
            struct Medium;
            #[state]
            struct High;
            #[state]
            struct VeryHigh;
        }
    }

    #[register(auto_increment)]
    mod pupdr {
        #[field(width = 2, read, write, auto_increment)]
        mod pupd0 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd1 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd2 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd3 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd4 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd5 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd6 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd7 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd8 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd9 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd10 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd11 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd12 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd13 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd14 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
        #[field(width = 2, read, write, auto_increment)]
        mod pupd15 {
            #[state(reset)]
            struct None;
            #[state]
            struct PullUp;
            #[state]
            struct PullDown;
        }
    }
}
