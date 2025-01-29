use std::path::PathBuf;
use crate::args::{ Cli, CommandType};
use crate::dispatcher::{request::Request, router::route_request};
use crate::storage::storage_map::StorageMap;
use clap:: Parser;

pub fn init() {
    let args = Cli::parse();
    init_storage();

    let request = match args.comm_type {
        CommandType::List(_)      => Request::List,
        CommandType::Add(args)    => Request::Add { id: args.id, path: args.path },
        CommandType::Find(args)   => Request::Find { pinpoint: args.pinpoint, key: args.key, auto: args.auto },
        CommandType::Delete(args) => Request::Delete { id: args.id },
        CommandType::Clear(args)  => Request::Clear { force: args.force },
        CommandType::To(args)     => Request::To { path: args.pinpoint },
    };

    route_request(request);
}



fn init_storage() {
    let storage_file = PathBuf::from("data.json");
    let mut storage_map = StorageMap::new(storage_file).expect("Failed to initialize storage map");   
    // Example usage
    storage_map.insert("example".to_string(), PathBuf::from("/example/path")).expect("Failed to insert");
    let value = storage_map.get("example");
    println!("Value: {:?}", value);
}