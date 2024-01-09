pub mod region;
pub use region::{Region, get_regions};
pub mod client;
pub use client::Client;

use custom_error::custom_error;

custom_error!{pub WordstatError
    BadResponse                                     = "Response had bad structure",
    UnknownResponseCode{code:reqwest::StatusCode}   = "Unknown response code recieved: {code}",
    UnknownError                                    = "Unknown error has occured"
}
