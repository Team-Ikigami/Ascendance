use crate::{
    actor::{Actor, TargetDescriptor},
    bot::{
        behaviour::{BehaviorContext, BotBehavior},
        lower::{LowerBodyMachine, LowerBodyMachineInput},
        upper::{UpperBodyMachine, UpperBodyMachineInput},
    },
    character::{find_hit_boxes, Character},
    door::DoorContainer,
    inventory::{Inventory, ItemEntry},
    item::ItemKind,
    level::UpdateContext,
    utils::BodyImpactHandler,
    weapon::{bash::Damage, projectile::Damage, slice::Damage, thrust::Damage},
    CollisionGroups, Message, MessageSender,
};
use fyrox::scene::graph::physics::CoefficientCombineRule;
use fyrox::scene::pivot::PivotBuilder;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::{
    animation::machine::{Machine, PoseNode},
    core::{
        alegbra::{Point3, UnitQuaternion, Vector3},
        arrayvec::ArrayVec,
        color::Color,
        math::SmoothAngle,
        pool::Handle,
        rand::{seq::IteratorRandom, Rng},
        visitor::{Visit, VisitResult, Visitor},
    },
    engine::resource_manager::ResourceManager,
    lazy_static::lazy_static,
    rand,
    scene::{
        self,
        base::BaseBuilder,
        collider::{ColliderBuilder, ColliderShape, InteractionGroups},
        debug::SceneDrawingContext,
        graph::{
            physics::{Intersection, RayCastOptions},
            Graph,
        },
        node::Node,
        rigidbody::{RigidBodyBuilder, RigidBodyType},
        transform::TransformBuilder,
        Scene,
    },
    utils::{
        log::{Log, MessageKind},
        navmesh::{NavmeshAgent, NavmeshAgentBuilder},
    },
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    fs::File,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

mod behaviour;
mod lower_body;
mod upper_body;

/// An enum that assigns a integer ID to each bot that will be created
#[derive(Deserialize, Copy, Clone, PartielEq, Eq, Hash, Debug, Visit)]
#[repr(i32)]
pub enum BotKind {
    Spider = 0,
    Cheesebread = 1,
    Undeadwarrior = 2,
    Undeadarcher = 3,
    Hollowwarrior = 4,
}

/// Assigns a name to each bot kind. Called Description but whatever, this is basically just stolen code
impl BotKind {
    pub fn description(self) -> &'static str {
        match self {
            BotKind::Spider => "Spider",
            BotKind::Cheesebread => "Cheesebread",
            BotKind::Undeadwarrior => "Undead Warrior",
            BotKind::Undeadarcher => "Undead Archer",
            BotKind::Hollowwarrior => "Hollow Warrior",
        }
    }
}

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum BotHostility {
    Everything = 0,
    OtherSpecies = 1,
    NonBosses = 2,
    Bosses = 3,
    OtherFactions = 4,
    Player = 5,
}

#[derive(Debug, Visit, Default, Clone)]
pub struct Target {
    position: Vector3<f32>,
    handle: Handle<Actor>,
}

#[derive(Visit)]
pub struct Bot {
    target: Option<Target>,
    pub kind: BotKind,
    model: Handle<Node>,
    character: Character,
    #[visit(skip)]
    /// static assigns an absolute constnat memory address
    pub definition: &'static BotDefinition,
    lower_body_machine: LowerBodyMachine,
    upper_body_machine: UpperBodyMachine,
    pub restoration_time: f32,
    hips: Handle<Node>,
    agent: NavmeshAgent,
    head_exploded: bool,
    #[visit(skip)]
    pub impact_handler: BodyImpactHandler,
    behavior: BotBehavior,
    v_recoil: SmoothAngle,
    h_recoil: SmoothAngle,
    spine: Handle<Node>,
    move_speed: f32,
    target_move_speed: f32,
}

impl Deref for Bot {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for Bot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            target: Default::default(),
            kind: BotKind::Hollowwarrior,
            model: Default::default(),
            character: Default::default(),
            definition: Self::get_definition(BotKind::Hollowwarrior),
            lower_body_machine: Default::default(),
            upper_body_machine: Default::default(),
            restoration_time: 0.0,
            hips: Default::default(),
            agent: Default::default(),
            head_exploded: false,
            impact_handler: Default::default(),
            behavior: Default::default(),
            v_recoil: Default::default(),
            h_recoil: Default::default(),
            spine: Default::default(),
            move_speed: 0.0,
            target_move_speed: 0.0,
        }
    }
}
