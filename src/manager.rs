use crate::args::{ Cli, CommandType};
use crate::dispatcher::{request::Request, router::route_request};
use crate::storage::storage_map::StorageMap;
use clap:: Parser;

pub fn init() {
    let args = Cli::parse();
    let mut storage = StorageMap::new();

    let request = match args.comm_type {
        CommandType::List(_)      => Request::List,
        CommandType::Add(args)    => Request::Add { id: args.id, path: args.path },
        CommandType::Find(args)   => Request::Find { pinpoint: args.pinpoint, key: args.key},
        CommandType::Delete(args) => Request::Delete { id: args.id },
        CommandType::Clear(args)  => Request::Clear { force: args.force },
        CommandType::To(args)     => Request::To { path: args.pinpoint },
    };

    route_request(request, &mut storage);
}
