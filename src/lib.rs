#![warn(missing_docs)]
/*! Provides validation facilities for the 
[sounding-base](https://github.com/rnleach/sounding-base.git) crate.

See [examples](examples/validate.rs) for example of library use.

*/

//
// API
//
pub use error::{ValidationError, ValidationErrors};
pub use validate::validate;

//
// Internal use only
//
extern crate optional;
extern crate sounding_base;

mod error;
mod validate;
