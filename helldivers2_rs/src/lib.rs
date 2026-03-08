#![allow(dead_code)]
extern crate self as helldivers2_rs;

use std::sync::OnceLock;

#[macro_export]
macro_rules! impl_route {
    ($endpoint:literal, $fn_name:ident, $out_type:ty, $desc:literal) => {
        #[doc = $desc]
        #[doc = ""]
        #[doc = concat!("Endpoint: `", $endpoint, "`")]
        pub async fn $fn_name() -> $crate::prelude::Result<$out_type> {
            $crate::middleware::request_blocking::<$out_type>($endpoint).await
        }
    };
}

pub mod middleware;
pub mod models;
pub mod prelude;

// Application information from outside
/// X-Super-Client value for application identification. See helldivers-2/api.
static SUPER_CLIENT: OnceLock<String> = OnceLock::new();
/// X-Super-Contact value for application contact information. See helldivers-2/api.
static SUPER_CONTACT: OnceLock<String> = OnceLock::new();

/// Wrapper for the Helldivers 2 community driven API. This wrapper is blocking to ensure rate-limits.
/// Use this struct to query for information.
pub struct HellApi;

impl HellApi {
    /// Initialize the API wrapper with some needed information.
    /// See helldivers-2/api for details.
    pub fn init(client: &str, contact: &str) {
        let _ = SUPER_CLIENT.set(String::from(client));
        let _ = SUPER_CONTACT.set(String::from(contact));
    }
}
