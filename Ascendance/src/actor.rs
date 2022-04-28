use crate::bot::BotKind; 
use crate::{
    bot::Bot,
    character::Character,
    level::UpdateContext,
    player::Player
}; 
use fyrox::{ 
    core::{
        algebra::Vector3, 
        pool::{
            Handle,
            Pool
        }, 
        visitor::{
            Visit,
            VisitResult,
            Visitor
        }, 
    },
    resource::texture::Texture, 
    scene::Scene, 
}; 
use std::ops::{
    Deref,
    DerefMut
};
#[allow(clippy::large_enum_variant)] 
#[derive(Visit)] 
pub enum Actor  { 
     Bot (Bot), 
     Player (Player), 
}
impl Default for Actor  { 
     fn default() -> Self { 
        Actor :: Bot ( Default :: default ()) 
    } 
} 

macro_rules!  static_dispatch { 
    ($self:ident, $func:ident, $($args:expr),*) => { 
         match  $ self  { 
            Actor::Player(v)  =>  v.$func($($args),*), 
            Actor::Bot(v) => v.$func($($args),*), 
        } 
    }; 
} 

impl Actor { 
    pub fn can_be_removed(&self, scene: &Scene) -> bool { 
        static_dispatch!(self , can_be_removed, scene)
    }
     pub fn clean_up(&mut self, scene: &mut Scene) {
         static_dispatch!(self, clean_up, scene)
    }
}

impl Deref for Actor { 
     type Target = Character; 

     fn deref(& self) ->  & Self ::Target { 
         match   self  { 
            Actor :: Bot (v)  =>  v, 
            Actor :: Player (v)  =>  v, 
        } 
    } 
} 

impl   DerefMut   for   Actor  { 
     fn   deref_mut ( & mut   self ) ->  & mut   Self ::Target { 
         match   self  { 
            Actor :: Bot (v)  =>  v, 
            Actor :: Player (v)  =>  v, 
        } 
    } 
}
