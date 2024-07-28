use crate::{player::CombatPlayer, settlement::{encode_address, SettleMentInfo, WithdrawInfo}};
use serde::Serialize;
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

const ERROR_PLAYER_NOT_FOUND: u32 = 1;

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str{
        match e {
            ERROR_PLAYER_NOT_FOUND => "PlayerNotFound",
            _ => "Unknown"
        }
    }

    pub fn decode(params: [u64; 4]) -> Self {
        let command = (params[0] & 0xffffffff) as u32;
        Transaction {
            command,
            data: [params[1], params[2], params[3]]
        }
    }

    pub fn deposit(&self) -> u32 {
        let pid = [self.data[0], self.data[1]];
        let mut player = CombatPlayer::get_from_pid(&pid);
        let balance = self.data[3];
        match player.as_mut() {
            None => {
                let player = CombatPlayer::new_from_pid(pid);
                player.store();
            },
            Some(player) => {
                player.data.balance += balance;
                player.store();
            }
        }
        0
    }

    pub fn withdraw(&self, pkey: &[u64; 4]) -> u32 {
        let mut player = CombatPlayer::get_from_pid(&CombatPlayer::pkey_to_pid(pkey));
        match player.as_mut() {
            None => ERROR_PLAYER_NOT_FOUND,
            Some(player) => {
                let withdraw = WithdrawInfo::new(
                    0,
                    0,
                    0,
                    [player.data.balance as u64, 0, 0, 0],
                    encode_address(&self.data.to_vec()),
                    );
                SettleMentInfo::append_settlement(withdraw);
                player.data.balance = 0;
                player.store();
                0
            }
        }
    }

    pub fn process(&self, pid: &[u64; 4]) -> u32 {
        if self.command == TIMETICK {
            unsafe {STATE.counter += 1};
            0
        } else if self.command == WITHDRAW {
            self.withdraw(pid)
        } else if self.command == DEPOSIT {
            self.deposit()
        } else {
            unreachable!()
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
