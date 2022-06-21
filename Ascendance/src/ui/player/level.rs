use fyrox::{
    gui::{
        button::{ButtonBuilder, message},
        grid::GridBuilder,
        tree::{
            Tree,
            TreeBuilder,
            TreeRoot,
            TreeRootBuilder,
        },
        vector_image::{
            VectorImageBuilder,
            VectorImage,
            Primitive,
        },
        widget::WidgetBuilder,
        ttf::SharedFont,
        UserInterface,
        Thickness,
        UiNode,
    },
    scene::{
        sound::{SoundBuilder, Status},
    },
    engine::{
        resource_manager::ResourceManager,
        Engine,
    },
};
pub struct Tree {}
struct Base {
    base: CircleBuilder,
    // lines: Vec<HashMap<Perk, Perk>,
}

impl Tree {
    pub fn new(&self, ui: UserInterface) -> Self {
        let ctx = &mut ui.build_ctx();

        // VectorImageBuilder::new(WidgetBuilder::new())
        //     .with_primitives(vec![Primitive::Circle{
        //         center: Vector2::new(20.0, 20.0),
        //         radius: 10.0,
        //         segments: 16,
        //     }])
        // .build(ctx);

        let bolster_endurance_one;
        let bolster_health_one;
        let cleric_one;
        TreeRootBuilder::new(WidgetBuilder::new())
            .with_items([
                TreeBuilder::new(WidgetBuilder::new())
                    .with_items([])
                    .with_content(
                        TextBuilder::new(WidgetBuilder::new())
                            .with_text("Cleric")
                            .with_font(
                                SharedFont::new(Font::default_char())
                                    .set(),
                            )
                            .with_wrap(WrapMode::Word)
                            .build(ctx),
                    )
                    .with_expanded(true)
                    .with_always_show_expander(false)
                    .build(ctx),
                TreeBuilder::new(WidgetBuilder::new())
                    .with_items([])
                    .build(ctx),
            .build(ctx);
    }
}

enum PerkName {
    basic,
    str_1,
    dex_1,
    int_1,
    fth_1,
    end_1,
    hpitm_1,
]

#[derive(Deserialize)]
pub struct Perk {
    type: String,
    stat: String,
    value: i32,
}
#[derive(Deserialize)]
pub struct LevelTreeDefinition {
    nodeId: String,
    nodeType: String,
    bonuses: Vec<Perk>,
    connectedTo: Array<String>,
    connectedFrom: String,
    cost: i32,
}
#[derive(Deserialize)]
#[derive(Default)]
pub struct LevelTreeDefinitionContainer {
    map: HashMap<PerkName, LevelTree>
}
impl LevelTreeDefinitionContainer {
    pub fn new() -> Self {
        let file = File::open("data/configs/levelling.ron").unwrap();
        ron::de::from_reader(file).unwrap()
    }
}
lazy_static! {
    static ref TREE: LevelTreeDefinitionContainer: LevelTreeDefinitionContainer::new();
}

impl LevelTree {
    pub fn get_definition(name: PerkName) -> &`static LevelTreeDefinition {
        TREE.map.get(&name).unwrap()
    }
    pub async fn new(
        name: PerkName,
        resource_manager: ResourceManager,
        ui: UserInterface,
    ) {}
}
