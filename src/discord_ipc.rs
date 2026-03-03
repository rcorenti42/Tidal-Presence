use byteorder::{LittleEndian, WriteBytesExt};
use serde_json::json;
use std::io::Write;
use std::os::windows::fs::OpenOptionsExt;
use std::{fs::OpenOptions, io};

const DISCORD_PIPE: &str = r"\\?\pipe\discord-ipc-0";

pub struct DiscordClient {
    pipe: std::fs::File,
}

impl DiscordClient {
    pub fn connect(app_id: &str) -> io::Result<Self> {
        let pipe = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(0)
            .open(DISCORD_PIPE)?;

        let mut client = Self { pipe };

        let handshake = json!({
            "v": 1,
            "client_id": app_id
        });

        client.send_frame(0, &handshake)?;

        Ok(client)
    }

    fn send_frame(&mut self, op: u32, payload: &serde_json::Value) -> io::Result<()> {
        let data = payload.to_string().into_bytes();
        let mut header = vec![];

        header.write_u32::<LittleEndian>(op)?;
        header.write_u32::<LittleEndian>(data.len() as u32)?;

        self.pipe.write_all(&header)?;
        self.pipe.write_all(&data)?;

        Ok(())
    }

    pub fn set_activity(
        &mut self,
        title: &str,
        artist: &str,
        start: Option<i64>,
        end: Option<i64>,
    ) -> io::Result<()> {
        let activity = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": {
                    "type": 2,
                    "details": title,
                    "state": artist,
                    "timestamps": {
                        "start": start,
                        "end": end
                    },
                    "assets": {
                        "large_image": "tidal",
                        "large_text": "Listening on TIDAL"
                    }
                }
            },
            "nonce": "1"
        });

        self.send_frame(1, &activity)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        let payload = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": null
            },
            "nonce": "2"
        });

        self.send_frame(1, &payload)
    }
}