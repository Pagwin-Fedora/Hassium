extern crate rust_sc2;
extern crate proc_macro;

use std::hash::Hash;

use rust_sc2::{*, prelude::*, player::{GameResult, Race}, unit::Unit, ids::UnitTypeId};

///Trait that any type which provides functionality which modules may depend on must implement
pub trait Provider<Provided> where Provided:Clone{
    fn get_data(&self)->Provided;
    fn subscribe(&mut self, callback:Box<dyn FnMut(Provided)->()>);
}

///Trait that any struct that wishes to listen to Events must implement
pub trait Listener{
    fn on_start(&mut self)->SC2Result<()>{Ok(())}
    fn on_step(&mut self, iteration:usize)->SC2Result<()>{Ok(())}
    fn on_end(&mut self, result:GameResult)->SC2Result<()>{Ok(())}
    fn on_event(&mut self, event:Event)->SC2Result<()>{Ok(())}
}

///Trait for structs that wish to take ownership of and manage units
pub trait UnitManager<Id> where Id: Hash{
    fn receive_unit(&mut self, unit:Unit)->Id;
    fn relinquish_unit(&mut self, unit:Id)->Unit;
}

///Trait that must be implemented by any module that wants to manage workers so they'll do things
///like speed mine
pub trait WorkerManager{
    ///method that will get called when the manager is taking over a worker
    fn take_worker(&mut self,unit:Unit);
    ///method that will get called when the manager needs to provide a worker to complete a task
    fn provide_worker(&mut self, position:Point2)->Unit;
}

///Trait that build orders need to implement for the sake of Hassium not letting whatever handle
///the build order and because I'll probably realize what I need to implement here eventually
pub trait BuildOrder{}

///Enum that is used by modules to pass messages to the framework
enum Message{RequestUnit(UnitTypeId), QueryUnit(Fn())}

///The actual bot framework that shoots all the events to the listeners
#[bot]
pub struct Hassium{
    //no list of Dependencies because I realized it doesn't help the situation at all as
    //dependencies need to be matched up to dependants individually
    ///A list of modules that are listening to events
    listeners:Vec<Box<dyn Listener>>,
    unit_manager:Box<dyn UnitManager>,
    //use mpsc to have hassium receive messages because I actually don't know how else to do it
}
impl Player for Hassium{
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Zerg)
            .with_name("Hassium")
            .raw_crop_to_playable_area(true)
            .raw_affects_selection(false)
    }
    fn on_start(&mut self)->SC2Result<()>{
        let mut ret:SC2Result<()> = Ok(());
        for elem in self.listeners.iter_mut(){
            ret = ret.and(elem.on_start());
        }
        ret
    }
    fn on_step(&mut self, iteration:usize)->SC2Result<()>{
        Ok(())
    }
    fn on_end(&self, result:GameResult)->SC2Result<()>{
        Ok(())
    }
    fn on_event(&mut self, event:Event)->SC2Result<()>{
        Ok(())
    }
}
impl Hassium {
    fn handle_message(){

    }
}
