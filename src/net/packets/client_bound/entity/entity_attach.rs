use crate::build_packet;
use crate::net::packets::packet::ClientBoundPacketImpl;
use tokio::io::{AsyncWrite, AsyncWriteExt, Result};

pub struct EntityAttach {
    entity_id: i32,
    vehicle_id: i32,
    leash: i8,
}

#[async_trait::async_trait]
impl ClientBoundPacketImpl for EntityAttach {
    async fn write_to<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<()> {
        let buf = build_packet!(
            0x1B,
            self.entity_id, // not a varint
            self.vehicle_id,
            self.leash,
        );
        writer.write_all(&buf).await
    }
}