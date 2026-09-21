pub mod new_room;
pub use new_room::procesar as procesar_new_room;

pub mod invite;
pub use invite::procesar as procesar_invite;

pub mod join_room;
pub use join_room::procesar as procesar_join_room;

pub mod room_users;
pub use room_users::procesar as procesar_room_users;

pub mod room_text;
pub use room_text::procesar as procesar_room_text;

pub mod leave_room;
pub use leave_room::procesar as procesar_leave_room;
