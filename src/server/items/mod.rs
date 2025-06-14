use crate::net::packets::packet::SendPacket;
use crate::server::items::etherwarp::handle_ether_warp;
use crate::server::player::Player;

pub mod item_stack;
mod etherwarp;

/// List of items available to be used
/// TODO, more
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Item {
    AspectOfTheVoid,
    DiamondPickaxe,
    SpiritSceptre,
}

impl Item {
    
    pub fn on_right_click(&self, player: &Player) -> anyhow::Result<()> {
        match self {
            Item::AspectOfTheVoid => {

                let server = &player.server_mut();
                let world = &server.world;
                let entity = player.get_entity(world)?;
                // test
                // CustomPayload {
                //     channel: "rustclear".to_string(),
                //     data: vec![30],
                // }.send_packet(player.client_id, &server.network_tx)?;

                if player.is_sneaking {
                    handle_ether_warp(player, &server.network_tx, world, entity)?;
                }

                // let pos = raycast_first_solid_block(world, entity, 60.0);
                // println!("pos raycasted {:?}", pos)
            }
            Item::SpiritSceptre => {
                // spawn bats, they copy yaw and pitch of player, idk the speed or whatever but
                // when they hit a solid block they blow up in like 10 block radius (or square) or something
            }
            _ => {}
        }
        Ok(())
    }

}