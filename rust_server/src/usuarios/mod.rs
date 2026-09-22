pub mod identify;
pub use identify::procesar as procesar_identify;

pub mod status;
pub use status::procesar as procesar_status;

pub mod users;
pub use users::procesar as procesar_users;

pub mod text;
pub use text::procesar as procesar_text;

pub mod public_text;
pub use public_text::procesar as procesar_public_text;

pub mod disconnect;
pub use disconnect::procesar as procesar_disconnect;
