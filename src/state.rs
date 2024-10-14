use crate::{player::CombatPlayer, settlement::SettlementInfo};
use lazy_static::lazy_static;
use serde::Serialize;
use sha2::Sha256;
use sha2::Digest;
use zkwasm_rest_abi::WithdrawInfo;
use zkwasm_rest_abi::MERKLE_MAP;
use zkwasm_rust_sdk::require;
use crate::game::Game;

const TIMETICK: u32 = 0;
const WITHDRAW: u32 = 1;
const REGISTER: u32 = 2;
const PLACE: u32 = 3;
const RESURRECT: u32 = 4;

pub struct Transaction {
    pub command: u32,
    pub data: [u64; 3],
}

const NO_ERROR: u32 = 0;
const ERROR_PLAYER_NOT_FOUND: u32 = 1;
const PLAYER_IN_GAME: u32 = 2;
const INVALID_PLACE: u32 = 3;
const ERROR_PLAYER_ALREADY_ACTIVITED: u32 = 4;
const PLAYER_IS_DEAD: u32 = 5;


lazy_static!(
    static ref HASHER:Sha256 = Sha256::new();
);

fn encode_activity(act: usize, value: u32) -> u64 {
    ((act as u64) << 32) + (value as u64)
}

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str{
        match e {
            ERROR_PLAYER_NOT_FOUND => "PlayerNotFound",
            PLAYER_IN_GAME => "PlayerIsInAnotherGame",
            INVALID_PLACE => "InvalidBetNumber",
            ERROR_PLAYER_ALREADY_ACTIVITED => "PlayerAlreadyExist",
            PLAYER_IS_DEAD => "PlayerIsDead",
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

    pub fn register(&self) -> u32 {
        let pid = [self.data[0], self.data[1]];
        let mut player = CombatPlayer::get_from_pid(&pid);
        match player.as_mut() {
            None => {
                let mut player = CombatPlayer::new_from_pid(pid);
                player.data.power += 1000;
                player.data.previous = encode_activity(1, 20);
                player.store();
                NO_ERROR
            },
            Some(_) => {
                ERROR_PLAYER_ALREADY_ACTIVITED
            }
        }
    }

    pub fn resurrect(&self) -> u32 {
        let pid = [self.data[0], self.data[1]];
        let mut player = CombatPlayer::get_from_pid(&pid);
        match player.as_mut() {
            None => {
                ERROR_PLAYER_NOT_FOUND
            },
            Some(player) => {
                player.data.balance = player.data.balance / 2;
                player.data.power += 1000;
                player.data.previous = encode_activity(1, 20);
                player.store();
                NO_ERROR
            }
        }
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
                } if player.data.power == 0 {
                    return PLAYER_IS_DEAD;
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
            zkwasm_rust_sdk::dbg!("new rand is {:?}\n", {self.data[1]});
            zkwasm_rust_sdk::dbg!("new rand bytes {:?}\n", {rand.to_le_bytes()});
            let mut hasher = HASHER.clone();
            hasher.update(rand.to_le_bytes());
            let v = hasher.finalize();
            let checkseed = u64::from_be_bytes(v[24..32].try_into().unwrap());
            zkwasm_rust_sdk::dbg!("v is {:?}\n", checkseed );
            if state.rand_commitment !=0 {
                unsafe { zkwasm_rust_sdk::require(state.rand_commitment == checkseed) };
            }
            state.rand_commitment = self.data[1];
            unsafe { STATE.settle(rand) };
            0
        } else if self.command == WITHDRAW {
            self.withdraw(pid)
        } else if self.command == REGISTER {
            self.register()
        } else if self.command == RESURRECT{
            self.resurrect()
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

#[derive(Serialize)]
pub struct UserState<'a> {
    player: Option<CombatPlayer>,
    global: &'a State,
}



impl State {
    pub fn initialize() {
        let state = unsafe { &mut STATE };
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&[0, 0, 0, 0]);
        if !data.is_empty() {
            let mut data = data.iter_mut();
            state.rand_commitment = *data.next().unwrap();
            state.counter = *data.next().unwrap();
        }
    }

    pub fn snapshot() -> String {
        let state = unsafe { &STATE };
        serde_json::to_string(&state).unwrap()
    }

    pub fn preempt() -> bool {
        let state = unsafe { &STATE };
        if state.counter % 100 == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn rand_seed() -> u64 {
        unsafe { STATE.rand_commitment }
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


    pub fn get_state(pkey: Vec<u64>) -> String {
        let player = CombatPlayer::get_from_pid(&CombatPlayer::pkey_to_pid(&pkey.try_into().unwrap()));
        serde_json::to_string(&player).unwrap()
    }

    pub fn store(&self) {
        let state = unsafe { &STATE };
        let mut v = Vec::with_capacity(2);
        v.push(state.rand_commitment);
        v.push(state.counter);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&[0, 0, 0, 0], v.as_slice());
    }

    pub fn flush_settlement() -> Vec<u8> {
        let data = SettlementInfo::flush_settlement();
        unsafe {STATE.store()};
        data
    }
}
