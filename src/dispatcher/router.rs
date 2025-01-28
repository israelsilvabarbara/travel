use crate::commands::{ add_command, 
                       clear_command, 
                       delete_command, 
                       find_command, 
                       list_command, 
                       to_command    };

use super::request::Request;



pub fn route_request(request: Request) {
    match request {
        Request::Add { id, path }             => add_command::execute(id, path),
        Request::Clear { force }              => clear_command::execute(force),
        Request::Delete { id }                => delete_command::execute(id),
        Request::Find { pinpoint, key, auto } => find_command::execute(pinpoint, key, auto),
        Request::List                         => list_command::execute(),
        Request::To { path }                  => to_command::execute(path),
    }
}
