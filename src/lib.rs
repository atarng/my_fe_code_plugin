#![feature(lazy_cell, ptr_sub_ptr)]

use skyline::install_hook;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;

use engage::gamedata::item::ItemData;

#[unity::hook("App", "Unit", "ClassChange")]
pub fn unit_classchange(this: &Unit, job: &JobData, item: Option<&ItemData>, _method_info : u64)
{
    let mut level_to_set_to = this.fields.level;
    let preserve_internal_level = this.fields.internal_level;
    if this.fields.level > 20 {
        level_to_set_to = this.fields.level - 20
    }

    else if this.get_job().get_max_level() == 20 && this.fields.level == 20 {
        if this.get_job().is_high() {
            level_to_set_to = 1
        }
        else {
            level_to_set_to = 10
        }
    }

    println!("running my class change");
    call_original!(this, job, item, _method_info);

    if let Some(item_unwrapped) = item {
        if item_unwrapped.fields.kind == 10 && item_unwrapped.fields.usetype == 24 {
            this.set_level(level_to_set_to.into());
            this.set_internal_level(preserve_internal_level.into())
        }
    }
}


#[skyline::main(name = "my_code_plugin")]
pub fn main() {
    println!("loading my_code_plugin");
    install_hook!(unit_classchange);
}
