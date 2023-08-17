macro_rules! chatrooms {
    [$($ids:literal)*] => {
        pub const CHATROOM_IDS: [u64; [0, $($ids,)*].len() - 1] = [ $($ids,)* ];
    };
}

chatrooms![
    1334858 // speckyyt
];
