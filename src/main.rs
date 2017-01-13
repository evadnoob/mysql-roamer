#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate mysql;
extern crate core;


use docopt::Docopt;
extern crate docopt;
use std::process::exit;

mod logging;
mod tables;
mod schemas;
mod repl;
mod banner;

static USAGE: &'static str = "
mysql_roamer - a modal database browser
Usage:
  mysql_roamer [options] [<args>] 

Options:
  -h --help        Show this screen.
  -v --verbose     verbose
  -n --no-connect  Just start, then stop, debug the startup process
  --version        Show version.

Some common forager commands are:
    help 
See 'mysql_roamer help <command>' for more information on a specific command.
";

fn main() {
    
    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    info!("starting up");

    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    info!("args {:?}", args);
    info!("arg vector: {:?}", args.get_vec("<args>"));

    
    if  !args.get_bool("-n") {
        banner::show();
        repl::start();
    }
    exit(0);
    
}

// unset '/etc/mysql/mysql.conf.d/mysqld.cnf' comment out 'bind-address'
// create database test;
// create user 'david'@'%' identified by 'david';
// grant all privileges on *.* to 'david'@'%';

// update mysql.user set host = '%' where user = 'david';
