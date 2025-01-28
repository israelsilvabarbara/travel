use clap::{
    Args,
    Parser,
    Subcommand,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Your application description here
pub struct Cli {
    #[command(subcommand)]
    pub comm_type: CommandType,
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// List all pinpoints
    #[command(alias = "l")]
    List(ListArgs),
    
    /// Pinpoint a path
    #[command(alias = "p")]
    Add(AddArgs),
    
    /// Find a path inside a pinpoint
    #[command(alias = "f")]
    Find(FindArgs),
    
    /// Delete a pinpoint
    #[command(alias = "d")]
    Delete(DeleteArgs),
    
    /// Clear all pinpoints
    #[command(alias = "c")]
    Clear(ClearArgs),
    
    /// Go to the specified pinpoint
    To(ToArgs),
}

#[derive(Args, Debug)]
pub struct ListArgs {}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Specify the ID of the item to pinpoint
    #[arg(short, long)]
    pub id: String,

    /// Specify the path of the pinpoint
    #[arg(short, long)]
    pub path: PathBuf,

}

#[derive(Args, Debug)]
pub struct FindArgs {
    /// Specify the pinpoint to find
    #[arg(default_value=".")]
    pub pinpoint:String,

    /// Specify the key for the find
    #[arg(short, long)]
    pub key: String,

    /// Automatically travel to the directory if only one result
    #[arg(short = 'a', long = "auto")]
    pub auto: bool
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Specify the ID of the item to delete
    pub id: String,
}


#[derive(Args,Debug)]
pub struct ClearArgs {
    // Force clear without asking
    #[arg(short, long)]
    pub force:bool,
}

#[derive(Args,Debug)]
pub struct ToArgs{
    pub pinpoint: String,
}