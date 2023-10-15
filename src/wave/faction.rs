use enum_map::EnumMap;

use crate::{data::faction::Faction, debug};

use super::{InstanceIndex, TeamIndex, Wave};

impl Wave<'_> {
    pub fn count_faction(&self, actor: TeamIndex, faction: Faction) -> usize {
        let mut count = 0;
        for i in self.get_team_indices(actor) {
            if self.heroes[i].faction == faction {
                count += 1;
            }
        }
        count
    }

    pub fn get_bond(&self, actor: InstanceIndex, faction: Faction) -> f32 {
        self.team_bonds[self.teams[actor]][faction]
    }

    pub fn set_bonds(&mut self) {
        for i in 0..self.team_bonds.len() {
            let mut bonds: EnumMap<Faction, f32> = EnumMap::default();
            for (f, v) in bonds.iter_mut() {
                match f {
                    Faction::WizardsEye => *v = self.bond_wizardseye(i),
                    Faction::DoomLegion => *v = self.bond_doomlegion(i),
                    Faction::Foresters => *v = self.bond_foresters(i),
                    Faction::SunsetSages => *v = self.bond_sunsetsages(i),
                    Faction::EternalSect => *v = self.bond_eternalsect(i),
                    Faction::DragonTribe => *v = self.bond_dragontribe(i),
                    Faction::HolyLightParliament => *v = self.bond_holylightparliament(i),
                    Faction::NamelessBrotherhood => *v = self.bond_namelessbrotherhood(i),
                    Faction::TheForgotten => *v = self.bond_theforgotten(i),
                    Faction::SwordHarborGuards => *v = self.bond_swordharborguards(i),
                    Faction::HiddenWave => *v = self.bond_hiddenwave(i),
                }
            }
            self.team_bonds[i] = bonds;
        }
    }

    pub fn bond_hiddenwave(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0, 0.0, 0.03, 0.06, 0.09, 0.12];
        let xfact = scale[self.count_faction(ti, Faction::HiddenWave)];
        debug!(
            "team {} has {} HiddenWave allies -> dmg * {}",
            ti,
            self.count_faction(ti, Faction::HiddenWave),
            xfact
        );
        xfact
    }

    pub fn bond_swordharborguards(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0, 0.0, 0.025, 0.05, 0.075, 0.10];
        let xfact = scale[self.count_faction(ti, Faction::SwordHarborGuards)];
        debug!(
            "team {} has {} SwordHarborGuards allies -> dmg * {}",
            ti,
            self.count_faction(ti, Faction::SwordHarborGuards),
            xfact
        );
        xfact
    }

    pub fn bond_theforgotten(&self, ti: TeamIndex) -> f32 {
        // work around for double scale
        let scale = vec![0.0, 0.0, 1.00, 2.00, 3.00, 4.00];
        let xfact = scale[self.count_faction(ti, Faction::TheForgotten)];
        debug!(
            "team {} has {} TheForgotten allies -> pierce * {}",
            ti,
            self.count_faction(ti, Faction::TheForgotten),
            xfact
        );
        xfact
    }

    pub fn bond_namelessbrotherhood(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0, 0.0, 0.20, 0.25, 0.30, 0.35];
        let xfact = scale[self.count_faction(ti, Faction::NamelessBrotherhood)];
        debug!(
            "team {} has {} NamelessBrotherhood allies -> pierce * {}",
            ti,
            self.count_faction(ti, Faction::NamelessBrotherhood),
            xfact
        );
        xfact
    }

    pub fn bond_holylightparliament(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0, 1.0, 1.06, 1.09, 1.12, 1.15];
        let xfact = scale[self.count_faction(ti, Faction::HolyLightParliament)];
        debug!(
            "team {} has {} HolyLightParliament allies -> heal * {}",
            ti,
            self.count_faction(ti, Faction::HolyLightParliament),
            xfact
        );
        xfact
    }

    pub fn bond_dragontribe(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0, 0.0, 0.025, 0.03, 0.035, 0.04];
        let xfact = scale[self.count_faction(ti, Faction::DragonTribe)];
        debug!(
            "team {} has {} DragonTribe allies -> critrate/mastery * {}",
            ti,
            self.count_faction(ti, Faction::DragonTribe),
            xfact
        );
        xfact
    }

    pub fn bond_eternalsect(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0, 1.0, 1.12, 1.16, 1.20, 1.24];
        let xfact = scale[self.count_faction(ti, Faction::EternalSect)];
        debug!(
            "team {} has {} EternalSect allies -> dmg * {}",
            ti,
            self.count_faction(ti, Faction::EternalSect),
            xfact
        );
        xfact
    }

    pub fn bond_sunsetsages(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0, 1.0, 1.12, 1.16, 1.20, 1.24];
        let xfact = scale[self.count_faction(ti, Faction::SunsetSages)];
        debug!(
            "team {} has {} SunsetSages allies -> leech * {}",
            ti,
            self.count_faction(ti, Faction::SunsetSages),
            xfact
        );
        xfact
    }

    pub fn bond_foresters(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0, 1.0, 1.06, 1.09, 1.12, 1.15];
        let xfact = scale[self.count_faction(ti, Faction::Foresters)];
        debug!(
            "team {} has {} Foresters allies -> atk/def * {}",
            ti,
            self.count_faction(ti, Faction::Foresters),
            xfact
        );
        xfact
    }

    pub fn bond_wizardseye(&self, ti: TeamIndex) -> f32 {
        let scale = vec![1.0, 1.0, 1.06, 1.09, 1.12, 1.15];
        let xfact = scale[self.count_faction(ti, Faction::WizardsEye)];
        debug!(
            "team {} has {} WizardsEye allies -> effect_hit * {}",
            ti,
            self.count_faction(ti, Faction::WizardsEye),
            xfact
        );
        xfact
    }

    pub fn bond_doomlegion(&self, ti: TeamIndex) -> f32 {
        let scale = vec![0.0, 0.0, 0.02, 0.03, 0.04, 0.05];
        let xfact = scale[self.count_faction(ti, Faction::DoomLegion)];
        debug!(
            "team {} has {} DoomLegion allies -> dmg red * {}",
            ti,
            self.count_faction(ti, Faction::DoomLegion),
            xfact
        );
        xfact
    }
}
