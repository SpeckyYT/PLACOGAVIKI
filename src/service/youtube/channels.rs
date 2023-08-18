macro_rules! channel {
    [$($k:tt $c:literal)*] => {
        #[allow(dead_code)]
        pub enum LiveSource {
            Channel(&'static str),
            Live(&'static str),
        }
        pub const LIVE_SOURCES: [LiveSource; 0 $(+ [stringify!($c)].len())*] = [
            $(
                channel!(# $k $c),
            )*
        ];
    };
    (# [channel] $t:literal) => {
        LiveSource::Channel($t)
    };
    (# [live] $t:literal) => {
        LiveSource::Live($t)
    };
}

channel![
    [channel] "UCXuqSBlHAE6Xw-yeJA0Tunw" // LTT channel
];
