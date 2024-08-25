use crate::{player::CombatPlayer, settlement::SettlementInfo};
use serde::Serialize;
use zkwasm_rest_abi::WithdrawInfo;
use zkwasm_rust_sdk::require;
use crate::game::Game;

const TIMETICK: u32 = 0;
const WITHDRAW: u32 = 1;
const DEPOSIT: u32 = 2;
const PLACE: u32 = 3;

pub struct Transaction {
    pub command: u32,
    pub data: [u64; 3],
}

const ERROR_PLAYER_NOT_FOUND: u32 = 1;
const PLAYER_IN_GAME: u32 = 2;
const INVALID_BET: u32 = 3;

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str{
        match e {
            ERROR_PLAYER_NOT_FOUND => "PlayerNotFound",
            PLAYER_IN_GAME => "PlayerIsInAnotherGame",
            INVALID_BET => "InvalidBetNumber",
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
                let amount = self.data[0] & 0xffffffff;
                unsafe {require(player.data.balance >= amount)};
                let withdrawinfo = WithdrawInfo::new(&self.data);
                SettlementInfo::append_settlement(withdrawinfo);
                player.data.balance -= amount;
                player.store();
                0
            }
        }
    }

    pub fn place(&self, place: u64, pkey: &[u64; 4], rand: u64) -> u32 {
        let mut player = CombatPlayer::get_from_pid(&CombatPlayer::pkey_to_pid(pkey));
        match player.as_mut() {
            None => ERROR_PLAYER_NOT_FOUND,
            Some(player) => {
                if player.data.placed != 0 {
                    return PLAYER_IN_GAME;
                } else if place == 0 && player.data.balance < place {
                    return INVALID_BET;
                } else {
                    let game = Game::new(&player, place, rand);
                    unsafe { STATE.new_game(game) };
                    player.data.placed = place;
                    player.store();
                    return 0
                }
            }
        }
    }

    pub fn process(&self, pid: &[u64; 4], sigr: &[u64; 4]) -> u32 {
        if self.command == TIMETICK {
            let state = unsafe { &mut STATE };
            state.counter += 1;
            let rand = self.data[0];
            state.rand_commitment = self.data[1];
            unsafe { STATE.settle(rand) };
            0
        } else if self.command == WITHDRAW {
            self.withdraw(pid)
        } else if self.command == DEPOSIT {
            self.deposit()
        } else if self.command == PLACE {
            let rand = sigr[0] ^ sigr[1] ^ sigr[2] ^ sigr[3];
            self.place(self.data[0], &pid, rand)
        } else {
            unreachable!()
        }
    }

}

#[derive (Serialize)]
pub struct State {
    rand_commitment: u64,
    counter: u64,
    games: Vec<Game>
}

pub static mut STATE: State  = State {
    rand_commitment: 0,
    counter: 0,
    games: vec![],
};

impl State {
    pub fn initialize() {
    }

    pub fn preempt() -> bool {
        return false;
    }

    pub fn settle(&mut self, rand: u64) {
        for game in self.games.iter_mut() {
            let final_rand = game.rand ^ rand;
            game.settle(final_rand);
        }
        self.games = vec![];
    }

    pub fn new_game(&mut self, game: Game) {
        self.games.push(game);
    }


    pub fn get_state(_pid: Vec<u64>) -> String {
        serde_json::to_string(unsafe {&STATE}).unwrap()
    }

    pub fn store(&self) {
    }

    pub fn flush_settlement() -> Vec<u8> {
        let data = SettlementInfo::flush_settlement();
        unsafe {STATE.store()};
        data
    }
}
