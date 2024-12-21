#![feature(lazy_cell, ptr_sub_ptr)]

use skyline::install_hook;

use engage::menu::BasicMenu;
use engage::menu::BasicMenuItem;

use engage::gamedata::JobData;
use engage::gamedata::item::ItemData;

use engage::gamedata::unit::GodUnit;
use engage::gamedata::unit::Unit;
use engage::gamedata::unit::UnitRing;

use unity::prelude::*;

#[unity::class("App", "UnitSelectRingMenu")]
pub struct UnitSelectRingMenu {
    // base: BasicMenu<UnitSelectRingMenu>, // I call it base but you can call it super if you want, it doesn't matter
    base: BasicMenu<BasicMenuItem>, // I call it base but you can call it super if you want, it doesn't matter
    close_event_handler: *const u8  // can also represent as u64, what matters is the type takes the same amount of space
}

impl UnitSelectRingMenu
{
    pub fn unitselectringmenu_entrustring(&mut self, _method_info : u64)
    {
        println!("[A] entrust ring.");
        unsafe { unitselectringmenu_entrustring(&self, None) }
    }

    pub fn unitselectringmenu_takeoffallrings(&mut self, _method_info : u64)
    {
        println!("[A] take off all rings");
        unsafe { unitselectringmenu_takeoffallrings(&self, None) }
    }

    pub fn unitselectringmenu_takeoffring(&mut self, unit: &Unit, _method_info : u64)
    {
        println!("[A] take off ring: {0}", unit.get_pid());
        unsafe { unitselectringmenu_takeoffring(&self, unit, None) }
    }
}

// // #[skyline::from_offset(0x1c616f0)]
// #[unity::from_offset("App", "UnitSelectRingMenu", "TakeOffAllRings")]
// fn unit_takeoffallrings(this: &UnitSelectRingMenu, _method_info : OptionalMethod);

// #[skyline::from_offset(0x1c61630)]
// #[unity::from_offset("App", "UnitSelectRingMenu", "TakeOffRing")]
// fn unit_takeoffring(this: &UnitSelectRingMenu, unit: &Unit, _method_info : OptionalMethod);

#[unity::hook("App", "UnitSelectRingMenu", "EntrustRing")]
pub fn unitselectringmenu_entrustring(this: &UnitSelectRingMenu, method_info: OptionalMethod) {
    println!("[B] entrust ring");
    call_original!(this, method_info);
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffAllRings")]
pub fn unitselectringmenu_takeoffallrings(this: &UnitSelectRingMenu, method_info: OptionalMethod) {
    println!("[B] take off all rings");
    call_original!(this, method_info);
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffRing")]
pub fn unitselectringmenu_takeoffring(this: &UnitSelectRingMenu, unit: &Unit, method_info: OptionalMethod) {
    println!("[B] take off ring: {0}", unit.get_pid());
    call_original!(this, unit, method_info);
}

#[unity::hook("App", "Unit", "ClassChange")]
pub fn unit_classchange(this: &Unit, job: &JobData, item: &ItemData, _method_info : u64)
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

    if item.fields.kind == 10 && item.fields.usetype == 24 {
        this.set_level(level_to_set_to.into());
        this.set_internal_level(preserve_internal_level.into())
    }
}

#[unity::hook("App", "Unit", "ClearGodUnit")]
pub fn unit_cleargod(this: &Unit, _method_info : u64)
{
    println!("[A] {} clear god unit", this.get_pid());
    call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "ClearRing")]
pub fn unit_clearring(this: &Unit, _method_info : u64)
{
    println!("[A] {0} clear ring", this.get_pid());
    call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "SetGodUnit")]
pub fn unit_setgod(this: &Unit, god_unit: &GodUnit, _method_info : u64)
{
    println!("[A] {0} set god unit", this.get_pid());
    // TryConnectGodUnit
    // ClearParent of old
    // SetParent of new
    // Unset Ring?

    let cached_ring = this.get_ring();
    call_original!(this, god_unit, _method_info);
    match cached_ring {
        Some(_value) => {
            this.set_ring(cached_ring.unwrap());
        }
        None => { println!("[A] No ring was assigned.") }
    }
}

#[unity::hook("App", "Unit", "SetRing")]
pub fn unit_setring(this: &Unit, ring: &UnitRing, _method_info : u64)
{
    println!("[A] {0} set ring", this.get_pid());
    
    let cached_god_unit = this.get_god_unit();
    call_original!(this, ring, _method_info);
    match cached_god_unit {
        Some(_value) => {
            this.set_god_unit(cached_god_unit.unwrap());
        }
        None => { println!("[A] No god unit was assigned") }
    }

    // unit_clearring(this, _method_info)
    // let location_of_ring = &this.fields.ring;
    // *location_of_ring = ring

}

#[skyline::main(name = "my_code_plugin")]
pub fn main() {
    println!("[A] loading my_code_plugin");
    install_hook!(unit_classchange);

    install_hook!(unitselectringmenu_entrustring);
    install_hook!(unitselectringmenu_takeoffallrings);
    install_hook!(unitselectringmenu_takeoffring);

    install_hook!(unit_cleargod);
    install_hook!(unit_clearring);
    install_hook!(unit_setgod);
    install_hook!(unit_setring);

//    install_hook!(unit_debug_ring_clear);
}
