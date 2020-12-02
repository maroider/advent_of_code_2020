use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(long, env)]
    pub token: String,
}

impl Opts {
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}

/// Like `concat!`, but insert a newline between every argument.
#[macro_export]
macro_rules! concat_newline {
    () => {};
    ( $arg:literal $(,)? ) => { $arg };
    ( $arg:literal , $( $argv:literal ),+  $(,)? ) => {
        core::concat!($arg, "\n", $crate::concat_newline!( $( $argv ),+ ))
    };
}
