use crate::build_packet;
use crate::net::packets::packet::ClientBoundPacketImpl;
use tokio::io::{AsyncWrite, AsyncWriteExt, Result};

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: String,
    pub name: String,
}

#[async_trait::async_trait]
impl ClientBoundPacketImpl for LoginSuccess {
    async fn write_to<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<()> {
        let buf = build_packet!(
            0x02,
            self.uuid,
            self.name,
        );
        writer.write_all(&buf).await
    }
}