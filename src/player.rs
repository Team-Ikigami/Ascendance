// Simple helper method to create a state supplied with PlayAnimation node.
fn create_play_animation_state(
    animation_resource: Model,
    name: &str,
    machine: &mut Machine,
    scene: &mut Scene,
    model: Handle<Node>,
) -> (Handle<Animation>, Handle<State>) {
    // Animations retargetting just makes an instance of animation and binds it to
    // given model using names of bones.
    let animation = *animation_resource
        .retarget_animations(model, scene)
        .get(0)
        .unwrap();
    // Create new PlayAnimation node and add it to machine.
    let node = machine.add_node(PoseNode::make_play_animation(animation));
    // Make a state using the node we've made.
    let state = machine.add_state(State::new(name, node));
    (animation, state)
}

pub struct AnimationMachineInput {
    // Whether a bot is walking or not.
    pub walk: bool,
}

pub struct AnimationMachine {
    machine: Machine,
}

impl AnimationMachine {
    // Names of parameters that will be used for transition rules in machine.
    const IDLE_TO_WALK: &'static str = "IdleToWalk";
    const WALK_TO_IDLE: &'static str = "WalkToIdle";

    pub async fn new(
        scene: &mut Scene,
        model: Handle<Node>,
        resource_manager: ResourceManager,
    ) -> Self {
        let mut machine = Machine::new();

        // Load animations in parallel.
        let (walk_animation_resource, idle_animation_resource) = rg3d::core::futures::join!(
            resource_manager.request_model(
                "data/models/paladin/walk.fbx",
                MaterialSearchOptions::RecursiveUp
            ),
            resource_manager.request_model(
                "data/models/paladin/idle.fbx",
                MaterialSearchOptions::RecursiveUp
            ),
        );

        // Now create two states with different animations.
        let (_, idle_state) = create_play_animation_state(
            idle_animation_resource.unwrap(),
            "Idle",
            &mut machine,
            scene,
            model,
        );

        let (walk_animation, walk_state) = create_play_animation_state(
            walk_animation_resource.unwrap(),
            "Walk",
            &mut machine,
            scene,
            model,
        );

        // Next, define transitions between states.
        machine.add_transition(Transition::new(
            // A name for debugging.
            "Idle->Walk",
            // Source state.
            idle_state,
            // Target state.
            walk_state,
            // Transition time in seconds.
            0.4,
            // A name of transition rule parameter.
            Self::IDLE_TO_WALK,
        ));
        machine.add_transition(Transition::new(
            "Walk->Idle",
            walk_state,
            idle_state,
            0.4,
            Self::WALK_TO_IDLE,
        ));

        // Define entry state.
        machine.set_entry_state(idle_state);

        Self { machine }
    }

    pub fn update(&mut self, scene: &mut Scene, dt: f32, input: AnimationMachineInput) {
        self.machine
            // Set transition parameters.
            .set_parameter(Self::WALK_TO_IDLE, Parameter::Rule(!input.walk))
            .set_parameter(Self::IDLE_TO_WALK, Parameter::Rule(input.walk))
            // Update machine and evaluate final pose.
            .evaluate_pose(&scene.animations, dt)
            // Apply the pose to the graph.
            .apply(&mut scene.graph);
    }
}
