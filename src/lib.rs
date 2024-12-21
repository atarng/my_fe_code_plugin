#![feature(lazy_cell, ptr_sub_ptr)]

use skyline::install_hook;

// use engage::dialog::yesno::YesNoDialog;
use engage::dialog::yesno::BasicDialogItemYes;

use engage::singleton::SingletonClass;

use engage::{
    menu::{BasicMenu, BasicMenuItem},
    gamedata::{Gamedata, JobData, PersonData, unit::{Unit, GodUnit, UnitRing}},
};

use engage::gamedata::item::ItemData;

use unity::prelude::*;

// ============================================
// ============================================

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

#[skyline::main(name = "my_code_plugin")]
pub fn main() {
    println!("[A] loading my_code_plugin");
    install_hook!(unit_classchange);
    //========================================

    install_hook!(unitselectringmenu_takeoffallrings);
    install_hook!(unitselectringmenu_takeoffring);

    install_hook!(confirmyesdialogitem_removeold);

    install_hook!(unitringpool_clearowner);
    install_hook!(unitringpool_setowner);

    // Does this get Called?
    install_hook!(unitring_changeowner);

    install_hook!(unit_tryconnectgodunit);
    install_hook!(unit_tryconnectgodunittocopy);
    install_hook!(unit_trydisconnectgodunit);
    install_hook!(unit_trydisconnectring);
    install_hook!(unit_cleargodunit);
    install_hook!(unit_cleargodunitfromcopy);
    install_hook!(unit_clearring);
    install_hook!(unit_setgod);
    install_hook!(unit_setring);

}

// =============================================
// RESEARCH AREA
// =============================================


#[unity::class("App", "RingData")]
pub struct RingData {}
impl Gamedata for RingData {}

#[unity::class("App", "UnitSelectRingMenu")]
pub struct UnitSelectRingMenu {
    base: BasicMenu<BasicMenuItem>, // I call it base but you can call it super if you want, it doesn't matter
    close_event_handler: *const u8  // can also represent as u64, what matters is the type takes the same amount of space
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffAllRings")]
pub fn unitselectringmenu_takeoffallrings(this: &UnitSelectRingMenu, method_info: OptionalMethod) {
    println!("[unitselectringmenu_takeoffallrings]");
    call_original!(this, method_info);
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffRing")]
pub fn unitselectringmenu_takeoffring(this: &UnitSelectRingMenu, unit: &Unit, method_info: OptionalMethod) {
    let _pid = unit.get_pid().inspect(|pid| println!("[unitselectringmenu_takeoffring] {pid}"))
        .expect("[unitselectringmenu_takeoffring] unit does not have pid.");
    call_original!(this, unit, method_info);
}


#[unity::class("App", "RingSelectConfirmDialog")]
pub struct RingSelectConfirmDialog {}

#[unity::class("RingSelectConfirmDialog", "ConfirmYesDialogItem")]
pub struct ConfirmYesDialogItem {
    base: BasicDialogItemYes
}

// This method is currently pretty buggy.
// #[unity::hook("App", "RingSelectConfirmDialog.ConfirmYesDialogItem")]
// Can't use unity::hook, because of nested struct/class
// 7101d60a10
#[skyline::hook(offset=0x01D60A10)]
pub fn confirmyesdialogitem_removeold(this: &ConfirmYesDialogItem, unit: Option<&Unit>, method_info: OptionalMethod) {
    // let cached_ring = unit.get_ring();
    // match cached_ring {
    //     Some(_value) => { println!("[confirmyesdialogitem_removeold] There is a ring available.") }
    //     None => { println!("[confirmyesdialogitem_removeold] No ring was assigned.") }
    // }
    // let cached_god = unit.get_god_unit();
    // match cached_god {
    //     Some(god_unwrapped) => {
    //         println!("[confirmyesdialogitem_removeold] God Exists: {0}",
    //                 god_unwrapped.fields.data.get_ascii_name().unwrap())
    //     }
    //     None => { println!("[confirmyesdialogitem_removeold] No God was assigned.") }
    // }

    match unit {
        Some(unit_unwrapped) => {
            // for some reason getting pid sems to crash.
            // let person = &unit_unwrapped.fields.person;
            // println!("[confirmyesdialogitem_removeold] person addres: {person:p}");
            let cached_ring = unit_unwrapped.get_ring();
            match cached_ring {
                Some(_ring_unwrapped) => {
                    // This does cause a panic attack when trying to take an emblem ring (from someone else?)
                    // println!("[confirmyesdialogitem_removeold] Ring Exists: {}", ring_unwrapped.data.name);
                    println!("[confirmyesdialogitem_removeold] Ring Exists");
                }
                None => { println!("[confirmyesdialogitem_removeold] No ring was assigned.") }
            }
            let cached_god = unit_unwrapped.get_god_unit();
            match cached_god {
                Some(_god_unwrapped) => { println!("[confirmyesdialogitem_removeold] God Exists.") }
                None => { println!("[confirmyesdialogitem_removeold] No God was assigned.") }
            }

            // let _crash_pid = unit_unwrapped.get_pid().
            //         inspect(|crash_pid| println!("[confirmyesdialogitem_removeold] pid: {crash_pid}"));
            // println!("[confirmyesdialogitem_removeold] level: {}", unit_unwrapped.level)
        } 
        None => { println!("[confirmyesdialogitem_removeold] no unit." ) }
    }

    call_original!(this, unit, method_info);
}


#[unity::class("App", "UnitRingPool")]
pub struct UnitRingPool {
     base: SingletonClass
}

// #[unity::hook("App", "UnitRingPool", "ClearOwner")]
// this: &UnitRingPool,
// static method
// clear owner: 7101c5d8a0
#[skyline::hook(offset=0x01C5D8A0)]
pub fn unitringpool_clearowner(ring: Option<&UnitRing>, method_info: OptionalMethod) {
    // println!("[unitringpool_clearowner] skip");
//===============================================
    match ring {
        Some(ring_unwrapped) => {
            println!("[unitringpool_clearowner] has ring: {0}", ring_unwrapped.data.name);
            match ring_unwrapped.owner {
                Some(owner_unwrapped) => {
                    let _pid = owner_unwrapped.get_pid().
                            inspect(|pid| println!("[unitringpool_clearowner] Owner being cleared: {pid}"));
                }
                None => {
                    println!("[unitringpool_clearowner] No Owner");
                }
            }
        }
        None => { println!("[unitringpool_clearowner] no ring"); }
    }
    call_original!(ring, method_info);
}

// #[unity::hook("App", "UnitRingPool", "SetOwner")]
// this: &UnitRingPool,
// static method
// clear owner: 7101c5d760
#[skyline::hook(offset=0x01C5D760)]
pub fn unitringpool_setowner(ring: &UnitRing, owner: Option<&Unit>, method_info: OptionalMethod) {
    println!("[unitringpool_setowner] ring: {0}", ring.data.name);

    let ring_previous_owner = ring.owner.is_some();
    let incoming_owner = owner.is_some();

    if ring_previous_owner && incoming_owner {
        println!("[unitringpool_setowner] Previous Owner: {0} Incoming Owner: {1}",
                ring.owner.unwrap().get_pid().unwrap(),
                owner.unwrap().get_pid().unwrap());
    } else if incoming_owner {
        println!("[unitringpool_setowner] NoPreviousOwner IncomingOwner: {0}",
                owner.unwrap().get_pid().unwrap());
    } else if ring_previous_owner {
        println!("[unitringpool_setowner] PreviousOwner: {0} NoIncomingOwner",
                ring.owner.unwrap().get_pid().unwrap());
    } else {
        println!("[unitringpool_setowner] NoPreviousOwner and NoIncomingOwner.");
    }

    call_original!(ring, owner, method_info);
}

#[unity::hook("App", "Unit", "TryConnectGodUnit")]
pub fn unit_tryconnectgodunit(this: &Unit, god_unit: &GodUnit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_tryconnectgodunit] {0}", this.get_pid().unwrap());
    return call_original!(this, god_unit, _method_info);
}

#[unity::hook("App", "Unit", "TryConnectGodUnitToCopy")]
pub fn unit_tryconnectgodunittocopy(this: &Unit, god_unit: &GodUnit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_tryconnectgodunittocopy] {0}", this.get_pid().unwrap());
    return call_original!(this, god_unit, _method_info);
}

#[unity::hook("App", "Unit", "TryDisconnectGodUnit")]
pub fn unit_trydisconnectgodunit(this: &Unit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_trydisconnectgodunit] {0}", this.get_pid().unwrap());
    return call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "TryDisconnectRing")]
pub fn unit_trydisconnectring(this: &Unit, _method_info : u64) -> Option<&'static UnitRing>
{
    println!("[unit_trydisconnectring] {0}", this.get_pid().unwrap());
    return call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "ClearGodUnit")]
pub fn unit_cleargodunit(this: &Unit, _method_info : u64)
{
    println!("[unit_cleargodunit] {}", this.get_pid().unwrap());
    call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "ClearGodUnitFromCopy")]
pub fn unit_cleargodunitfromcopy(this: &Unit, _method_info : u64)
{
    println!("[unit_cleargodunitfromcopy] {0} ", this.get_pid().unwrap());
    call_original!(this, _method_info);
}

// Used Pretty much everywhere:
// Involved in clearing unit for previews when selecting emblem/bond ring.
#[unity::hook("App", "Unit", "ClearRing")]
pub fn unit_clearring(this: &Unit, _method_info : u64)
{
    // println!("[unit_clearring] Unit: {0} SKIPCLEARRING", this.get_pid().unwrap());
    //===================================================================
    println!("[unit_clearring] Unit: {0} ClearRing", this.get_pid().unwrap());
    call_original!(this, _method_info);
}

// Only seeing this called on launch.
#[unity::hook("App", "UnitRing", "ChangeOwner")]
pub fn unitring_changeowner(this: &UnitRing, owner: &Unit, _method_info: u64)
{
    println!("[unitring_changeowner] Ring: {0}", this.data.name);
    match this.owner {
        Some(owner_unwrapped) => {
            println!("[unitring_changeowner] Previous Owner: {0}", owner_unwrapped.get_pid().unwrap())
        }
        None => {
            println!("[unitring_changeowner] No Owner");
        }
    }
    println!("[unitring_changeowner] Change owner to: {0}", owner.get_pid().unwrap());
    call_original!(this, owner, _method_info);
}

#[unity::hook("App", "Unit", "SetGodUnit")]
pub fn unit_setgod(this: &Unit, god_unit: &GodUnit, _method_info : u64)
{
    println!("[unit_setgod] {0}", this.get_pid().unwrap());
    call_original!(this, god_unit, _method_info);
}

// This part is a bit odd, since Ring already points to unit as its owner.
// Possibly that the Unit doesn't associate the ring as its ring?
#[unity::hook("App", "Unit", "SetRing")]
pub fn unit_setring(this: &Unit, ring: &UnitRing, _method_info : u64)
{
    let units_ring = this.get_ring();
    match units_ring {
        Some(unwrapped_ring) => {
            println!("[unit_setring] {0} has an existing ring: {1}", this.get_pid().unwrap(), unwrapped_ring.data.name);
        } None => {
            println!("[unit_setring] {0} does not have an existing ring.", this.get_pid().unwrap());
        }
    }

    match ring.owner {
        Some(owner_unwrapped) => {
            println!("[unit_setring] Ring {0} Owner: {1}", ring.data.name, owner_unwrapped.get_pid().unwrap());
        }
        None => {
            println!("[unit_setring] Ring {0} NoOwner", ring.data.name);
        }
    }

    call_original!(this, ring, _method_info);
}
