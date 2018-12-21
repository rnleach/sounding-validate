#![warn(missing_docs)]
/*! Provides validation facilities for the 
[sounding-base](https://github.com/rnleach/sounding-base.git) crate.

See [examples](examples/validate.rs) for example of library use.

*/

//
// API
//
pub use crate::error::{ValidationError, ValidationErrors};
pub use crate::validate::validate;

//
// Internal use only
//

mod error;
mod validate;
