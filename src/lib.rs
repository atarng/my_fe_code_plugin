#![feature(lazy_cell, ptr_sub_ptr)]

use skyline::{
    install_hook,
    patching::Patch,
};

use engage::gamedata::{
    unit::Unit,
    JobData,
    item::ItemData,
};

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

    // Allows reclass on CC is what I change to make it allow learning on class change
    Patch::in_text(0x01BE9508).bytes([0x04, 0x00, 0x00, 0x14]).unwrap();
}
