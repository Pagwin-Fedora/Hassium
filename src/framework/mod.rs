extern crate rust_sc2;
extern crate proc_macro;

use rust_sc2::{bot, Player,SC2Result, player::{GameResult, Race}, Event, PlayerSettings};

///Trait that any type which provides functionality which modules may depend on must implement
pub trait Dependency<Provided>{
    fn get_data(&self)->Provided;
}

///Trait that any bot module must implement
// I don't know what should own the dependency such that a dependency can have multiple dependants
// that can want to mutate it
pub trait Dependant<T>{
    //passing the Dependency type itself here doesn't work becuase of ownership and only a single
    //mutable reference being allowed to exist at a time so instead I'm passing a function that
    //will provide what the Dependant needs
    fn pass_dependency<D>(&mut self,provider:Box<dyn Fn()->T>) {}
}

///Trait that any struct that wishes to listen to Events must implement
pub trait Listener{
    fn on_start(&mut self, hassium:&mut Hassium)->SC2Result<()>{Ok(())}
    fn on_step(&mut self, hassium:&mut Hassium, iteration:usize)->SC2Result<()>{Ok(())}
    fn on_end(&mut self, hassium:&Hassium, result:GameResult)->SC2Result<()>{Ok(())}
    fn on_event(&mut self, hassium:&mut Hassium, event:Event)->SC2Result<()>{Ok(())}
}
///The actual bot framework that shoots all the events to the listeners
#[bot]
pub struct Hassium{
    //no list of Dependencies because I realized it doesn't help the situation at all as
    //dependencies need to be matched up to dependants individually
    listeners:Vec<Box<dyn Listener>>
}
impl Player for Hassium{
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Zerg)
            .with_name("Hassium")
            .raw_crop_to_playable_area(true)
            .raw_affects_selection(false)
    }
    fn on_start(&mut self)->SC2Result<()>{Ok(())}
    fn on_step(&mut self, iteration:usize)->SC2Result<()>{Ok(())}
    fn on_end(&self, result:GameResult)->SC2Result<()>{Ok(())}
    fn on_event(&mut self, event:Event)->SC2Result<()>{Ok(())}
}
