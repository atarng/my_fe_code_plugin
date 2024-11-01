#![feature(lazy_cell, ptr_sub_ptr)]

use skyline::install_hook;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;

use engage::gamedata::item::ItemData;

#[unity::hook("App", "Unit", "ClassChange")]
pub fn unit_classchange(this: &Unit, job: &JobData, item: &ItemData, _method_info : u64)
{
    println!("running my class change");
    call_original!(this, job, item, _method_info)
}


#[skyline::main(name = "my_code_plugin")]
pub fn main() {
    println!("loading my_code_plugin");
    install_hook!(unit_classchange);
}
