use crate::GameTime;
use crate::MessageSender;
use crate::actor::Actor;
use crate::actor::ActorContainer;
use crate::effects::EffectKind;
use crate::level::ballista::Ballista;
use crate::message::Message;
use crate::items::weapon::ray_hit;
use crate::items::weapon::sight::SightReaction;
use crate::items::weapon::Hit;
use crate::items::weapon::Weapon;
use crate::items::weapon::WeapoContainer;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::scene::sprite::Sprite;
use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::core::pool::Pool;
use fyrox::lazy_static::lazy_static;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::Visitor;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use fyrox::core::math::vector_to_quat;
use fyrox::core::math::Vector3Ext;
use fyrox::core::visitor::VisitResult;
use fyrox::engine::resource_manager::ResourceManager;
use fyrox::scene::graph::Graph;
use fyrox::scene::node::Node;
use fyrox::scene::Scene;
use serde::Deserialize;


/// Describes what type of projectile an actor gets hit by
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Hash, Visit)]
pub enum ProjectileKind {
    Arrow,
    Ballista,
    GreatArrow,
    Explosive
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Visit)]
pub enum Shooter {
    None,
    Actor(Handle<Actor>),
    Weapon(Handle<Weapon>)
}
impl Default for Shooter {
    fn default() -> Self {
        Self::None
    }
}

/// For forms of damage that splash such as explosives, this creates a radius and amount of effect
/// it has on nearby enemies. For projectiles like arrows, the Point is there to determine where it
/// hits?
#[derive(Deserialize, Copy, Clone, Debug, Visit)]
pub enum Damage {
    Splash {
        radius: f32,
        amount: f32
    },
    Point(f32),
}
// sets the default for the location of what it hits.
impl Default for Damage {
    fn default() -> Self {
        Self::Point(0.0)
    }
}
impl Damage {
    #[must_use]
    pub fn scale(&self, k: f32) -> Self {
        // Using * on a self makes it a pointer. Id love to know what K is and why it has the
        // function ".abs()
        match *self {
            Self::Splash { amount, radius } => Self::Splash {
                amount: amount * k.abs(),
                radius,
            },
            Self::Point(amount) => Self::Point(amount * k.abs()),
        }
    }
    pub fn amount(&self) -> f32 {
        *match self {
            Damage::Splash { amount, .. } => amount,
            Damage::Point(amount) => amount,
        }
    }
}

#[derive(Visit)]
pub struct Projectile {
    kind: ProjectileKind,
    model: Handle<Node>,
    /// Handle of rigid body assigned to projectile. Some projectiles, like tar bombs,
    /// arrows, plasma balls could have rigid body to detect collisions with
    /// environment. Some projectiles do not have rigid body - they're ray-based -
    /// interaction with environment handled with ray cast.
    body: Handle<Node>,
    dir: Vector3<f32>,
    lifetime: f32,
    rotation_angle: f32,
    pub owner: Shooter,
    initial_velocity: Vector3<f32>,
    /// Position of projectile on the previous frame, it is used to simulate
    /// continuous intersection detection from fast moving projectiles.
    last_position: Vector3<f32>,
    #[visit(skip)]
    definition: &'static ProjectileDefinition,
    #[visit(skip)]
    hits: HashSet<Hit>,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            kind: ProjectileKind::Arrow,
            model: Default::default(),
            dir: Default::default(),
            body: Default::default(),
            lifetime: 0.0,
            rotation_angle: 0.0,
            owner: Default::default(),
            initial_velocity: Default::default(),
            last_position: Default::default(),
            definition: Self::get_definition(ProjectileKind::Arrow),
            hits: Default::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct ProjectileDefinition {
    damage: Damage,
    speed: f32,
    lifetime: f32,
    /// Means that movement of projectile controlled by code, not physics.
    /// However projectile still could have rigid body to detect collisions.
    is_kinematic: bool,
    impact_sound: String,
    model: String,
}

#[derive(Deserialize, Default)]
pub struct ProjectileDefinitionContainer {
    map: HashMap<ProjectileKind, ProjectileDefinition>,
}

// Container makes this the initial handle for what gets read from the projectiles config files. It uses the ron crate to read the information maps.
impl ProjectileDefinitionContainer {
    pub fn new() -> Self {
        let file = File::open("data/configs/projectiles.ron").unwrap();
        ron::de::from_reader(file).unwrap()
    }
}

// lazy_static! is a macro that creates a static variable that always exists. This uses the ability to do type annotations to constants is used to make this
// constant the variables read from the config file. The type annotation is the struct that is used to read the data from the config file. This ::new() is
// what reads the data from the config file. It is a static method that returns a information to the struct that then uses the information in the constant.
// The ending of the ::new() doesnt end with a semicolon, so it is a usable even later as seen in Projectile::get_definition().
lazy_static! {
    static ref DEFINITIONS: ProjectileDefinitionContainer = ProjectileDefinitionContainer::new();
}

impl Projectile {
	// The lazy static variable that is used to read the data from the Projectile config files is used to call additional ron::de::from_reader() functions
	// ProjectileDefinitionContainer::new() doesnt end with a semicolon so this is possible. The -> &`static ProjectileDefinition` is returning the information
	// in a static lifetime of the ProjectileDefinition struct.
    pub fn get_definition(kind: ProjectileKind) -> &'static ProjectileDefinition {
        DEFINITIONS.map.get(&kind).unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        kind: ProjectileKind,
        resource_manager: ResourceManager,
        scene: &mut Scene,
        dir: Vector3<f32>,
        position: Vector3<f32>,
        owner: Shooter,
        initial_velocity: Vector3<f32>,
    ) -> Self {
		// Gets the information from the get_definition function using the kind variable that is necesary to call this function.
        let definition = Self::get_definition(kind);

		// adds the model of whatever the definition variable called the get_definitions with.
        let resource = resource_manager
            .request_model(definition.model.clone())
            .await
            .unwrap();
		// creates a new node with the model that was added to memory.
        let model = resource.instantiate_geometry(scene);
		// TODO: Read about this function
        let body = scene.graph.find_by_name(model, "Projectile");
		// Create a reference to the scene graphs "body" variable that was created a line ago.
        let body_ref = &mut scene.graph[body];
		// Using the reference just created, it makes a mutable local transformation (linear movement of the model) and sets the position somewhere, i would
		// assume to be the final position of the projectile. It then checks if the body variable is a rigid body, if it is, it moves the body to the position
		// with "lin_vel" which, with a bit of context clues can be assumed to be linear velocity which is always set to be the inital velocity. Useful for
		// projectiles because calculating the linear velocity and how it changes is annoying and unwanted for me and the computer
        body_ref.local_transform_mut().set_position(position);
        if let Some(body) = body_ref.cast_mut::<RigidBody>() {
            body.set_lin_vel(initial_velocity);
        }

        Self {
            lifetime: definition.lifetime,
            body,
            initial_velocity,
            dir: dir
                .try_normalize(std::f32::EPSILON)
                .unwrap_or_else(Vector3::y),
            kind,
            model,
            last_position: position,
            owner,
            definition,
            ..Default::default()
        }
    }

	// This function checks the lifetime variable of this struct to see if it is below or equal to absolute 0. Useful.
    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }

	// This function outright kills the projectile by setting the structs lifetime variable to be absolute 0. Useful for when you dont want the game to become
	// unplayably laggy. Reading a bit further showed me that it is useful to have this when an arrow hits its target ofc. Didnt think about that at all
	// before.
	pub fn kill(&mut self) {
		self.lifetime = 0.0;
	}

    pub fn update(
        &mut self,
        scene: &mut Scene,
        actors: &ActorContainer,
        weapons: &WeaponContainer,
        time: GameTime,
        sender: &MessageSender,
    ) {
        // Fetch current position of projectile.
        let (position, collider) = if self.body.is_some() {
			// creates reference to body of the projectile in the scene graph
            let body_ref = &scene.graph[self.body];
			// creates a variable of the position of the body of the projectile
            let position = body_ref.global_position();
			// creates a variable of the collider of the body of the projectile. The process is that for each of the children of the body_ref referenced nodes
			// it iterates a filter over the lower level nodes of the scene graph to see if it is a collider. Meaning it checks to see if the projectile has a collider node. It then checks the next node to see if it is a collider. It then does an unwrap or a default function. I think unwrap is a function that returns the value of the node if it is a collider and default continues to the next child node of the projectile..
            let collider = body_ref
                .children()
                .iter()
                .cloned()
                .filter(|c| scene.graph[*c].is_collider())
                .next()
                .unwrap_or_default();
            (position, collider)
        }
		// if the body of the projectile doesnt exist  then it iterates through the scene graph for the global position of the self.model variable, it also
		// makes the handle fo the node a NONE constant.
		else {
            (scene.graph[self.model].global_position(), Handle::NONE)
        };

		// no fucking idea what a ray hit even is but this creates a variable to see the last position, current position, the owner node of the projectile(?)
		// node, the weapon and the actors, the physics of the scene and the collider. I THINK THIS IS FOR BOUNCY SHIT.
        let ray_hit = ray_hit(
            self.last_position,
            position,
            self.owner,
            weapons,
            actors,
            &mut scene.graph.physics,
            collider,
        );

		// Creates several variables if the weapon::hit exists.  It uses the ray_hit set of variables. It uses the hit.position to set the variable of the 
		// ray_hit  position. It then uses the hit.normal vairable to se the ray_hit.normal variable. It then creates a blood effect variable if the actor 
		// that gets hit is_some() TODO: FIND OUT ABOUT THAT
		//
		// At the end of the two parts of the if statement there is a ( ) that calls a couple variables. They are called in order to correspond to the (effect_position, effect_normal, effect_kind). They are NOT RANDOM OR ODD.
        let (effect_position, effect_normal, effect_kind) = if let Some(hit) = ray_hit {
            let position = hit.position;
            let normal = hit.normal;
            let blood_effect = hit.actor.is_some();

			// It inserts another of the hit struct to the projectile.hits variable.
            self.hits.insert(hit);
			// The projectile is killed :'(
            self.kill();

			// Makes the just created position variable and normal variable. It checks if the blood_effect variable is real. If it is, it creates a blood
			// splatter effect. If not it does nothing and uses the BigImpact part of the EffectKind enum.
            (
                position,
                normal,
                if blood_effect {
                    EffectKind::BloodSpray
                } else {
                    EffectKind::BigImpact
                },
            )
        }
		// if  the previous big If statement is false, then it uses the get_position in this impl and calls it with a mutable scene.graph. It then uses the y 
		// variable of the 3d plane. It sets the last part of the variable to be the non bloodspray impact
		else {
            (
                self.get_position(&scene.graph),
                Vector3::y(),
                EffectKind::Impact,
            )
        };

		// CONTINUE HERE
        // Movement of kinematic projectiles are controlled explicitly.
        if self.definition.is_kinematic {
            let total_velocity = self.dir.scale(self.definition.speed);

            // Special case for projectiles with rigid body.
            if self.body.is_some() {
                scene.graph[self.body]
                    .local_transform_mut()
                    .offset(total_velocity);
            } else {
                // We have just model - move it.
                scene.graph[self.model]
                    .local_transform_mut()
                    .offset(total_velocity);
            }
        }

        if let Some(sprite) = scene.graph[self.model].cast_mut::<Sprite>() {
            sprite.set_rotation(self.rotation_angle);
            self.rotation_angle += 1.5;
        }

        // Reduce initial velocity down to zero over time. This is needed because projectile
        // stabilizes its movement over time.
        self.initial_velocity.follow(&Vector3::default(), 0.15);

        self.lifetime -= time.delta;

        if self.lifetime <= 0.0 {
            sender.send(Message::CreateEffect {
                kind: effect_kind,
                position: effect_position,
                orientation: vector_to_quat(effect_normal),
            });

            sender.send(Message::PlaySound {
                path: PathBuf::from(self.definition.impact_sound.clone()),
                position: effect_position,
                gain: 1.0,
                rolloff_factor: 4.0,
                radius: 3.0,
            });
        }

        for hit in self.hits.drain() {
            let damage = self
                .definition
                .damage
                .scale(hit.hit_box.map_or(1.0, |h| h.damage_factor));

            let critical_shot_probability = match self.owner {
                Shooter::Weapon(weapon) => {
                    if hit.actor.is_some() {
                        sender.send(Message::SightReaction {
                            weapon,
                            reaction: SightReaction::HitDetected,
                        });
                    }

                    weapons[weapon].definition.base_critical_shot_probability
                }
                Shooter::Turret(_) => 0.01,
                _ => 0.0,
            };

            match damage {
                Damage::Splash { radius, amount } => sender.send(Message::ApplySplashDamage {
                    amount,
                    radius,
                    center: position,
                    who: hit.who,
                    critical_shot_probability,
                }),
                Damage::Point(amount) => sender.send(Message::DamageActor {
                    actor: hit.actor,
                    who: hit.who,
                    hitbox: hit.hit_box,
                    amount,
                    critical_shot_probability,
                }),
            }
        }

        self.last_position = position;
    }

    pub fn get_position(&self, graph: &Graph) -> Vector3<f32> {
        graph[self.model].global_position()
    }

    fn clean_up(&mut self, scene: &mut Scene) {
        if scene.graph.is_valid_handle(self.body) {
            scene.graph.remove_node(self.body);
        } else {
            scene.graph.remove_node(self.model);
        }
    }

    pub fn resolve(&mut self) {
        self.definition = Self::get_definition(self.kind);
    }
}

#[derive(Default, Visit)]
pub struct ProjectileContainer {
    pool: Pool<Projectile>,
}

impl ProjectileContainer {
    pub fn new() -> Self {
        Self { pool: Pool::new() }
    }

    pub fn add(&mut self, projectile: Projectile) -> Handle<Projectile> {
        self.pool.spawn(projectile)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Projectile> {
        self.pool.iter_mut()
    }

    pub fn update(
        &mut self,
        scene: &mut Scene,
        actors: &ActorContainer,
        weapons: &WeaponContainer,
        time: GameTime,
        sender: &MessageSender,
    ) {
        for projectile in self.pool.iter_mut() {
            projectile.update(scene, actors, weapons, time, sender);
            if projectile.is_dead() {
                projectile.clean_up(scene);
            }
        }

        self.pool.retain(|proj| !proj.is_dead());
    }

    pub fn resolve(&mut self) {
        for projectile in self.pool.iter_mut() {
            projectile.resolve();
        }
    }
}
