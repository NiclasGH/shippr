use std::env;

use config::File;

use crate::Result;

struct AppConfig;
//impl AppConfig {
//    pub fn new() -> Result<Self> {
//        let path = env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from("."));
//
//        // environment underscores are kept: AXUM_BASE_URL etc BUT, it cant work with _ in variable names.
//        // Therefor we use __ to separate the normal nesting and variable names
//        let config = config::Config::builder()
//            .add_source(File::with_name(&format!(
//                "deployment"
//            )))
//            .build()?
//            .try_deserialize()?;
//
//        Ok(config)
//    }
//}

