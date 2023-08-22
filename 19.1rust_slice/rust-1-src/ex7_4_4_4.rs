fn main() {
    crate::sound::instrument::clarinet();
    sound::instrument::inner::clarinet();
}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::voice::hello();
        }
        pub mod inner {
            pub fn clarinet() {
                super::super::voice::hello();
            }
        }
    }

    pub mod voice {
        pub fn hello() {
            println!("voice.hello");
        }
    }
}