use serde::Serialize;
use crate::player::{CombatPlayer, Settle};


#[derive(Clone)]
pub struct CommitmentInfo ([u64; 2]);

impl CommitmentInfo {
    pub fn new(c0: u64, c1: u64) -> Self {
        CommitmentInfo([c0, c1])
    }
}

#[derive(Serialize, Clone)]
pub struct Game {
    pub player: [u64; 2],
    pub bet: u64,
    pub rand: u64,
    pub result: Option<[u32; 2]>,
}

impl Game {
    pub fn new(player: &CombatPlayer, bet: u64, rand: u64) -> Self {
        Game {
            player: player.player_id,
            rand,
            bet,
            result: None
        }
    }
    pub fn settle(&mut self, info: u64) {
        let mut player = CombatPlayer::get_from_pid(&self.player).unwrap();
        let command = player.data.previous >> 32;
        if command == 1 { // FIGHT
            if player.data.placed == 0 {
                let damage = player.data.previous & 0xff;
                if player.data.power > damage {
                    player.data.power -= damage;
                } else {
                    player.data.power = 0;
                }
            } else {
                player.data.balance = player.data.balance / 2;
            }
        } else {
            if player.data.placed == 0 {
                player.data.balance += (player.data.previous >> 8) & 0xff;
            } else {
                player.data.power += (player.data.previous) & 0xff;
            }
        }
        player.data.previous = info & 0x1ffffffff;
        player.data.placed = 0; // last game has been settled
        player.store();
    }
}
