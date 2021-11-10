use crate::generate_controller;

mod create;
mod read;

use create::new_tag;
use create::new_todo;

generate_controller!(TodoCtrl, "/todos", new_todo, new_tag);
