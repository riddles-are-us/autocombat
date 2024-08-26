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
        let result = [(info % 6) as u32, ((info >> 8) % 6) as u32]; // (player, server)'s number;
        self.result = Some (result);
        let mut player = CombatPlayer::get_from_pid(&self.player).unwrap();
        if result[0] >= result[1] {
            player.settle_rewards(self.bet);
        }
        player.data.previous = info;
        player.data.placed = 0; // last game has been settled
        player.store();
    }
}
