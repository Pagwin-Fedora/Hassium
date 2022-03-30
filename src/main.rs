extern crate rust_sc2;


use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rust_sc2::{prelude::*, units::*};
use std::{iter::Sum, collections::HashSet};
use std::time;
type SRes = SC2Result<()>;

//TODO replace this with home dir from the dirs crate and maybe should go into the cache dir not
//the home dir
const REPLAY_DIR:&str = "/home/fedora/";

#[bot]
#[derive(Default)]
struct Hassium{
    latest_hatch:Option<Unit>,
    latest_sat: u8
}
impl Hassium{
    fn next_hatch_loc(units:&PlayerUnits, expands: Vec<Point2>) -> Point2{

        let taken = units.townhalls.iter()
            .chain(units.placeholders.iter())
            .map(Unit::position)
            .collect::<std::collections::HashSet<Point2>>();

        let taken_midpoint:Point2 = Sum::<Point2>::sum(taken.iter().map(|v|v.clone()));
        //this is dumb
        let taken_midpoint = taken_midpoint / taken.len() as f32;

        let available = expands.iter()
            .filter(|v|!taken.contains(v));
        let dist = |point:&Point2| taken_midpoint.distance(point);
        available.min_by(|point1,point2|dist(point1).partial_cmp(&dist(point2)).unwrap_or(std::cmp::Ordering::Equal)).unwrap().clone()
    }
    fn currently_ordering(units:&Units, ids:&HashSet<AbilityId>) -> HashSet<AbilityId>{
        //the fact that this has to be so complicated is dumb
        ids.iter()
            .filter(|a| units.iter()
                .any(|u|u.orders().iter()
                    .any(|o|o.ability == **a)
                    ))
            .map(AbilityId::clone)
            .collect::<HashSet<AbilityId>>()
    }
}
impl Player for Hassium{
    fn on_start(&mut self) -> SC2Result<()> {
        self.latest_hatch = Some(self.units.my.townhalls.first().unwrap().clone());
        self.latest_sat = 12;
        Ok(())
    }
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Zerg)
            .with_name("Hassium")
            .raw_crop_to_playable_area(true)
            .raw_affects_selection(false)
    }
    fn on_step(&mut self, iteration: usize) -> SRes {
        
        let OVIE_BUILD:HashSet<AbilityId> = {
            let mut t = HashSet::new();
            t.insert(AbilityId::LarvaTrainOverlord);
            t
        };

        while self.latest_sat < 16 {
            if self.can_afford(UnitTypeId::Drone, true) {
                match self.units.my.larvas.pop(){
                    Some(larva)=>{
                        larva.train(UnitTypeId::Drone,false);
                    }
                    None=> return Ok(())
                }
            }
            else if self.supply_left < 1 {
                //should probably split off this condition into a method
                if Hassium::currently_ordering(&self.units.my.units, &OVIE_BUILD).is_empty() {
                    self.units.my.larvas.pop().unwrap().train(UnitTypeId::Overlord, false);
                }
            }
        }
        
        if self.minerals >= 350 && !self.units.my.larvas.is_empty() && self.latest_sat == 16 {

            let dist = |point:Point2| self.latest_hatch.clone().unwrap()
                .position().distance(point);

            let cmp = |f:&f32,s:&f32|f32::partial_cmp(f,s).unwrap();

            self.units.my.workers.iter()
                .map(Unit::position)
                .map(dist)
                .min_by(cmp)
                .unwrap();
            
        }

        Ok(())
    }
}

fn main() -> SRes {
    let location = format!("{}{}.SC2Replay",
        REPLAY_DIR,
        time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH).unwrap()
            .as_secs());
    let options ={ 
        let mut o = LaunchOptions::default();
        o.save_replay_as = Some(location.as_str());
        o    
    };
    run_vs_computer(&mut Hassium::default(), 
        Computer::new(Race::Zerg,Difficulty::Hard,Some(AIBuild::Rush)),
        "BerlingradAIE",
        options
        )
}
