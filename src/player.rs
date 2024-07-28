use serde::Serialize;
use crate::MERKLE_MAP;

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerInfo (pub [u64; 2]);

impl PlayerInfo {
    pub fn new(pid: &[u64;4]) -> Self {
        PlayerInfo([pid[1], pid[2]])
    }
    pub fn from_raw(a:u64, b:u64) -> Self {
        PlayerInfo([a, b])
    }
    pub fn to_key(&self) -> [u64; 4] {
        [self.0[0], self.0[1], 0xff00, 0xff01]
    }
}

#[derive(Debug, Serialize)]
pub struct Player {
    #[serde(skip_serializing)]
    pub player_id: PlayerInfo,
    pub balance: u64,
}

impl Player {
    pub fn store(&self) {
        let mut data = Vec::with_capacity(1);
        data.push(self.balance);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&self.player_id.to_key(), data.as_slice());
        zkwasm_rust_sdk::dbg!("end store player\n");
    }

    pub fn new(player_id: &[u64; 4]) -> Self {
        Self {
            player_id: PlayerInfo::new(player_id),
            balance: 0,
        }
    }

    pub fn get(pid: &[u64; 4]) -> Option<Self> {
        let kvpair = unsafe { &mut MERKLE_MAP };
        let player = PlayerInfo::new(pid); 
        let data = kvpair.get(&player.to_key());
        if data.is_empty() {
            None
        } else {
            let balance = data[0].clone();
            let p = Player {
                player_id: player.clone(),
                balance,
            };
            Some(p)
        }
    }
}
