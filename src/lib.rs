#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};

use skyline::install_hook;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;

use engage::gamedata::ItemData;

void App.Unit$$ClassChange
               (App_Unit_o *__this,App_JobData_o *job,App_ItemData_o *item,MethodInfo *method)
#[unity::hook("App", "Unit", "ClassChange")]
pub fn unit_classchange(this: &Unit, job: &JobData, item: &ItemData, _method_info : u64)
{
    println!("running my class change");
    call_original!(this, job, item, _method_info)
}


#[unity::hook("App", "Unit", "ExpToSkillPoint")]
pub fn unit_exptoskillpoint(this: &Unit, exp: i32, _method_info : u64) -> i32
{
    let config = CONFIG.lock().unwrap();

    if this.m_GodUnit.is_some()
    {
        (exp as f32 * config.multiplier_emblem_ring) as i32
    }
    else if this.m_Ring.is_some()
    {
        (exp as f32 * config.multiplier_bond_ring) as i32
    }
    else 
    {
        (exp as f32 * config.multiplier_no_ring) as i32
    }
}

#[skyline::main(name = "my_code_plugin")]
pub fn main() {
    println!("loading my_code_plugin");
    install_hook!(unit_classchange);
}
