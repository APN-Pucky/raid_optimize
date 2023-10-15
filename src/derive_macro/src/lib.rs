extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

#[proc_macro_derive(Cooldown)]
pub fn cooldown(input: TokenStream) -> TokenStream {
    // add a get_cooldown method to the struct
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Cooldown for #name {
            fn get_cooldown(&self) -> u32 {
                self.cooldown
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PassiveSkill)]
pub fn passive_skill(input: TokenStream) -> TokenStream {
    // add a get_cooldown method to the struct
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Selector {
            SELECT : Select = Select::None;
        }
        impl TYPE for #name {
            TYPE : SkillType = SkillType::Passive;
        }
        impl Cooldown for #name {
            fn get_cooldown(&self) -> u32 {
                0
            }
        }
        impl Execute for #name {
            fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
                panic!("Passive skill should not be executed");
            }
        }
    };
    gen.into()
}
