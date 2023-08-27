use enum_map::EnumMap;
use strum::EnumProperty;

use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave, TeamIndex};


impl<const LEN:usize> Wave<'_,LEN> {

    pub fn count_faction(&self, actor : TeamIndex, faction : Faction) -> usize {
        let mut count = 0;
        for i in self.get_team_indices(actor) { 
            if self.heroes[i].faction == faction {
                count+=1;
            }
        }
        count
    }

    pub fn set_bonds(&mut self) {
        for i in 0..self.team_bonds.len() {
            let mut bonds : EnumMap<Faction,f32> = EnumMap::default();
            for (f,v) in bonds.iter_mut() {
                match f {
                    Faction::WizardsEye => *v = self.bond_wizardseye(i),
                    Faction::DoomLegion => *v = self.bond_doomlegion(i),
                    _ => *v = 0.0,
                }
            }
            self.team_bonds[i] = bonds;
        }
    }

    pub fn bond_foresters(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0,1.0,1.06,1.09,1.12,1.15];
        let xfact = scale[self.count_faction(ti,Faction::Foresters)];
        debug!("team {} has {} Foresters allies -> atk/def * {}", ti, self.count_faction(ti,Faction::Foresters), xfact);
        xfact
    }

    pub fn bond_wizardseye(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0,1.0,1.06,1.09,1.12,1.15];
        let xfact = scale[self.count_faction(ti,Faction::WizardsEye)];
        debug!("team {} has {} WizardsEye allies -> effect_hit * {}", ti, self.count_faction(ti,Faction::WizardsEye), xfact);
        xfact
    }

    pub fn bond_doomlegion(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0,0.0,0.02,0.03,0.04,0.05];
        let xfact = scale[self.count_faction(ti,Faction::DoomLegion)];
        debug!("team {} has {} DoomLegion allies -> dmg red * {}", ti, self.count_faction(ti,Faction::DoomLegion), xfact);
        xfact
    }
}