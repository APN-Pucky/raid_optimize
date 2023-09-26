use crate::{debug, indent, data::{effect::{Effect, is_debuff}, skill::{Skill, get_selection}, faction::Faction}};

use super::{InstanceIndex, Wave};


impl Wave<'_> {
    pub fn before_action(&mut self, actor : InstanceIndex) {
        //let a = self.instances[actor];//get_ally_indices(actor);
        //let e = self.get_enemies_indices(actor);
        //let e = self.get_enemies(actor);
        //let (a,e) = if actor.team {
        //        (&mut self.allies[actor.index], &mut self.enemies)
        //    }else {
        //        (&mut self.enemies[actor.index], &mut self.allies)
        //    };
        debug!("before {} acts", self.name(actor));
        indent!({
            self.on_turn_start_marville(actor);
            // apply effects 
            // apply heal
            self.dot_heal(actor);
            // apply bleed
            self.dot_bleed(actor);
            // apply HP burning
            self.dot_hp_burning(actor);

            self.turn_reduce_cooldowns(actor);
        })
    }

    pub fn after_action(&mut self, actor :InstanceIndex) {
        debug!("after {} acts", actor);
        indent!({
            self.after_action_tifya(actor);
            if self.get_faction(actor) == Faction::DragonTribe && self.bonds_counter[actor] < 5 {
                self.bonds_counter[actor] += 1;
            }
            self.set_turn_meter(actor,0.0);
            self.effect_reduce(actor);
            self.shield_reduce(actor);
            self.team_acted[self.teams[actor]] = true;
        })
    }


    pub fn act(&mut self, actor : InstanceIndex) {
        debug!("{} acts", self.fmt(actor));
        indent!({
            //
            if !self.is_alive(actor) {
                debug!("{} is dead -> can't take turn", self.fmt(actor));
                return;
            }
            self.before_action(actor);
            if !self.is_alive(actor) {
                debug!("{} is dead now -> can't take turn", self.fmt(actor));
                return;
            }
            // choose action
            let skills : Vec<&Skill> = self.get_active_skills(actor);
            debug!("{} has active skills:", self.name(actor));
            indent!({
                for s in skills.iter() {
                    debug!("{}", s);
                }
            });

            let skill :&Skill = self.get_player_of_instance(actor).pick_skill(self, actor,&skills);

            debug!("{} chooses {:?}", self.name(actor), skill);
            indent!({
                // get targets
                let ts = get_selection(self,skill.select, actor); 
                if !ts.is_empty() {
                    let target : InstanceIndex = self.get_player_of_instance(actor).pick_target(self, actor, &skill, &ts);
                    //
                    self.pre_execute_skill(actor, target,skill );
                    // apply skill
                    self.execute_skill(skill, actor, target);
                }
                else {
                        // TODO maybe not even provide this option as active skill
                        debug!("{} has no valid targets for {}", self.fmt(actor), skill);
                        return;
                }
            });
            // finish
            self.after_action(actor);
        })
    }
}