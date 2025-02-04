use crate::commands::{ add_command, 
                       clear_command, 
                       delete_command, 
                       find_command, 
                       list_command, 
                       to_command    };
use crate::storage::storage_map::StorageMap;

use super::request::Request;



pub fn route_request(request: Request, storage: &mut StorageMap) {
    match request {
        Request::Add { id, path }             => add_command::execute(storage, id, path),
        Request::Clear { force }              => clear_command::execute(storage, force),
        Request::Delete { id }                => delete_command::execute(storage, id),
        Request::Find { pinpoint, key } => find_command::execute(storage, pinpoint, key),
        Request::List                         => list_command::execute(storage),
        Request::To { path }                  => to_command::execute(storage, path),
    }
}
