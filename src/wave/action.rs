use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}}};

use super::{InstanceIndex, Wave};


impl<const LEN:usize> Wave<'_,LEN> {
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
            // apply effects 
            // apply heal
            self.dot_heal(actor);
            // apply bleed
            self.dot_bleed(actor);
            // apply HP burning
            self.dot_hp_burning(actor);

            self.reduce_cooldowns(actor);
        })
    }

    pub fn after_action(&mut self, actor :InstanceIndex) {
        debug!("after {} acts", actor);
        indent!({
            self.set_turn_meter(actor,0.0);
            self.effect_reduce(actor);
            self.shield_reduce(actor);
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

            let skill :&Skill = self.get_player_of_instance(actor).pick_skill(self, actor, &skills);

            debug!("{} chooses {}", self.name(actor), skill);
            indent!({
                // get targets
                match get_targets(&skill, actor, self) {
                    Some(ts) => {
                        let target : InstanceIndex = self.get_player_of_instance(actor).pick_target(self, actor, &skill, &ts);
                        // apply skill
                        execute_skill(skill, actor, target, self);
                    },
                    None => {
                        // TODO maybe not even provide this option as active skill
                        debug!("{} has no valid targets for {}", self.fmt(actor), skill);
                        return;
                    },
                }
            });
            // finish
            self.after_action(actor);
        })
    }
}