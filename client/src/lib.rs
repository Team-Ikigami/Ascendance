#![deny(unsafe_code)]

pub mod addr;
pub mod cmd;
pub mod error;

pub use crate::error::Error;
pub use authc::AuthClientError;
pub use common_net::msg::ServerInfo;
pub use specs::{
    join::Join,
    saveload::{Marker, MarkerAllocator},
    Builder,
    DispatcherBuilder,
    Entity as EcsEntity,
    ReadStorage,
    World,
    WorldExt,
};
use crate::addr::ConnectionArgs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
