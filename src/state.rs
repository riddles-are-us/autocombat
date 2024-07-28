use crate::settlement::{encode_address, SettleMentInfo, WithdrawInfo};
use serde::Serialize;
use crate::player::{Player, PlayerInfo};
use crate::game::{Game, CommitmentInfo, Content};

const TIMETICK: u32 = 0;
const COMMITCARDS: u32 = 1;
const PROVIDECARDS: u32 = 2;
const WITHDRAW: u32 = 3;
const DEPOSIT: u32 = 4;

pub struct Transaction {
    pub command: u32,
    pub data: [u64; 3],
}

impl Transaction {
    pub fn decode(params: [u64; 4]) -> Self {
        let command = (params[0] & 0xffffffff) as u32;
        Transaction {
            command,
            data: [params[1], params[2], params[3]]
        }
    }

    pub fn deposit(&self) -> bool {
        let balance = self.data[3];
        let player_id = PlayerInfo::from_raw(self.data[0], self.data[1]);
        let mut player = Player::get(&player_id.to_key());
        match player.as_mut() {
            None => {
                let player = Player {
                    player_id,
                    balance,
                };
                player.store();
                true
            },
            Some(player) => {
                player.balance += balance;
                player.store();
                true
            }
        }
    }

    pub fn withdraw(&self, pid: &[u64; 4]) -> bool {
        let mut player = Player::get(pid);
        match player.as_mut() {
            None => false,
            Some(player) => {
                let withdraw = WithdrawInfo::new(
                    0,
                    0,
                    0,
                    [player.balance as u64, 0, 0, 0],
                    encode_address(&self.data.to_vec()),
                    );
                SettleMentInfo::append_settlement(withdraw);
                player.balance = 0;
                player.store();
                true
            }
        }
    }

    pub fn process(&self, pid: &[u64; 4]) -> bool {
        if self.command == TIMETICK {
            unsafe {STATE.counter += 1};
            true
        } else if self.command == COMMITCARDS {
            let state = unsafe {&mut STATE};
            let game = &mut state.game;
            let content = Content {
                player: PlayerInfo::new(pid),
                commitment: CommitmentInfo::new(self.data[0], self.data[1]),
                content: None
            };
            game.add_commitment(content)
        } else if self.command == PROVIDECARDS {
            let state = unsafe {&mut STATE};
            let game = &mut state.game;
            let data = self.data[0].to_le_bytes();
            if game.add_content(PlayerInfo::new(pid), data.to_vec()) {
                if game.fullfilled() {
                    game.settle()
                };
                true
            } else {
                false
            }
        } else if self.command == WITHDRAW {
            self.withdraw(pid)
        } else if self.command == DEPOSIT {
            self.deposit()
        } else {
            false
        }
    }
}

#[derive (Serialize)]
pub struct State {
    counter: u64,
    game: Game
}

static mut STATE: State  = State {
    counter: 0,
    game: Game {
        game_id: 0,
        contents: vec![]
    }
};

impl State {
    pub fn initialize() {
    }
    pub fn get_state(_pid: Vec<u64>) -> String {
        serde_json::to_string(unsafe {&STATE}).unwrap()
    }
    pub fn store() {
    }
}
