use serde::{Serialize, Serializer, ser::SerializeSeq};
use crate::player::PlayerInfo;


#[derive(Clone)]
pub struct CommitmentInfo ([u64; 2]);

fn serialize_player_info<S>(value: &PlayerInfo, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(value.0.len()))?;
        for e in value.0.iter() {
            seq.serialize_element(&e.to_string())?;
        }
        seq.end()
    }

// Custom serializer for `u64` as a string.
fn serialize_commitment_info<S>(value: &CommitmentInfo, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(value.0.len()))?;
        for e in value.0.iter() {
            seq.serialize_element(&e.to_string())?;
        }
        seq.end()
    }

#[derive(Serialize, Clone)]
pub struct Content {
    #[serde(serialize_with="serialize_player_info")]
    pub player: PlayerInfo,
    #[serde(serialize_with="serialize_commitment_info")]
    pub commitment: CommitmentInfo,
    pub content: Option<Vec<u8>> // card contents
}

impl CommitmentInfo {
    pub fn new(c0: u64, c1: u64) -> Self {
        CommitmentInfo([c0, c1])
    }
}

#[derive(Serialize, Clone)]
pub struct Game {
    pub game_id: u64,
    pub contents: Vec<Content>,
}

impl Game {
    pub fn add_commitment(&mut self, content: Content) -> bool {
        if self.contents.len() < 2 {
            return false;
        }
        for c in self.contents.iter_mut() {
            if c.player == content.player {
                return false
            }
        }
        self.contents.push(content);
        true
    }
    pub fn add_content(&mut self, pi: PlayerInfo, data: Vec<u8>) -> bool {
        for c in self.contents.iter_mut() {
            if c.player == pi {
                c.content = Some(data);
                return true
            }
        }
        return false
    }
    pub fn fullfilled(&self) -> bool {
        if self.contents.len() != 2 {
            return false
        }
        if self.contents[0].content.is_some()
            && self.contents[1].content.is_some() {
                true
        } else {
            false
        }
    }
    pub fn settle(&mut self) {
        todo!()
    }
}
