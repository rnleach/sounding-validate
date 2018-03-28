#![warn(missing_docs)]
/*! Provides validation facilities for the 
[sounding-base](https://github.com/rnleach/sounding-base.git) crate.

See [examples](examples/validate.rs) for example of library use.

*/

//
// API
//
pub use validate::validate;
pub use error::{ValidationError, ValidationErrors};

//
// Internal use only
//

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate sounding_base;

mod error;
mod validate;
