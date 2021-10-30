mod super::Enemy::Snake;
use Snake::{
    hydra::{model, load},
    commongreen::{model, load},
    commonbrown::{model, load},
    commonred::{model, load},
    god::{model, load},
};
use ron::*;

struct hurt {
    // Struct made of function
    cred: func,
};
struct create {
    cred: func,
};

impl create {
    fn cred() {
        load::new()
            .token(rand::randinteger(1...101))
            .health(rand::randinteger(place))
            .build()
    }
}
impl hurt {
    fn cred() {
        ron::parse("loadedsnake.ron");
        ron::search(snake.token
    }
}
