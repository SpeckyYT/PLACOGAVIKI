use dotenv::dotenv;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use lazy_static::lazy_static;
use crate::service::debug;
use crate::service::DebugType;

pub const DOTENV_PATH: &str = ".env";

macro_rules! dotenv {
    {$($key:ident $(,)?)*} => {
        // pub const DOTENV_KEYS: [&str; 0 $(+ [stringify!($key)].len())*] = [ $(stringify!($key),)* ];

        pub struct Dotenv {
            $(
                pub $key: String,
            )*
        }

        impl Default for Dotenv {
            fn default() -> Dotenv {
                if dotenv().is_err() {
                    assert!(fs::write(DOTENV_PATH, "").is_ok());
                }

                let mut dotenv_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(DOTENV_PATH)
                    .unwrap();

                $(
                    if std::env::var(stringify!($key).to_uppercase()).is_err() {
                        assert!(writeln!(dotenv_file, "\n{}=\"\"", stringify!($key).to_uppercase()).is_ok());
                    }
                )*

                Dotenv {
                    $(
                        $key: std::env::var(stringify!($key).to_uppercase()).unwrap_or_default().to_string()
                    )*
                }
            }
        }

        impl Dotenv {
            pub fn print(&self, process: String) {
                $(
                    debug(
                        process,
                        DebugType::Custom {
                            title: stringify!($key).to_string(),
                            extra: self.$key.clone(),
                        }
                    );
                )*
            }
        }
    };
}

lazy_static!(
    pub static ref DOTENV: Dotenv = Dotenv::default();
);

dotenv! {
    discord_token,
}
