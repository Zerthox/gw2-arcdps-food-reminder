pub use super::buff::{Buff, BuffState, Food, Utility};
use arc_util::game::Player;
pub use arc_util::game::{Profession, Specialization};
use serde::{Deserialize, Serialize};
use std::cmp;

// TODO: track buff duration & reset to unset when duration runs out?

/// Struct representing a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Player this entry corresponds to.
    pub player: Player,

    /// Current food buff applied to the player.
    pub food: Buff<Food>,

    /// Current utility buff applied to the player.
    pub util: Buff<Utility>,
}

impl Entry {
    /// Creates a new entry.
    pub const fn new(player: Player) -> Self {
        Self {
            player,
            food: Buff::new(BuffState::Unset, 0, 0),
            util: Buff::new(BuffState::Unset, 0, 0),
        }
    }

    /// Sets all unset buffs to none.
    pub fn unset_to_none(&mut self, time: u64, event_id: u64) {
        if self.food.state == BuffState::Unset {
            self.food.update(BuffState::None, time, event_id);
        }
        if self.util.state == BuffState::Unset {
            self.util.update(BuffState::None, time, event_id);
        }
    }

    /// Applies a food buff to the player.
    ///
    /// Returns `true` if this update changed the buff state.
    pub fn apply_food(&mut self, food: Food, time: u64, event_id: u64) -> bool {
        self.food.update(BuffState::Known(food), time, event_id)
    }

    /// Applies a unknown food buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_unknown_food(&mut self, id: u32, time: u64, event_id: u64) -> bool {
        self.food.update(BuffState::Unknown(id), time, event_id)
    }

    /// Removes the current food buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_food(&mut self, food: Option<Food>, time: u64, event_id: u64) -> bool {
        let changed = match (food, self.food.state) {
            (_, BuffState::Unset) | (None, BuffState::Unknown(_)) => true,
            (Some(removed), BuffState::Known(applied)) => removed == applied,
            _ => false,
        };
        if changed {
            self.food.update(BuffState::None, time, event_id)
        } else {
            false
        }
    }

    /// Applies an utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_util(&mut self, util: Utility, time: u64, event_id: u64) -> bool {
        self.util.update(BuffState::Known(util), time, event_id)
    }

    /// Applies an unknown utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_unknown_util(&mut self, id: u32, time: u64, event_id: u64) -> bool {
        self.util.update(BuffState::Unknown(id), time, event_id)
    }

    /// Removes the current utility buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_util(&mut self, util: Option<Utility>, time: u64, event_id: u64) -> bool {
        let changed = match (util, self.util.state) {
            (_, BuffState::Unset) | (None, BuffState::Unknown(_)) => true,
            (Some(removed), BuffState::Known(applied)) => removed == applied,
            _ => false,
        };
        if changed {
            self.util.update(BuffState::None, time, event_id)
        } else {
            false
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player
    }
}

impl Eq for Entry {}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.player.partial_cmp(&other.player)
    }
}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.player.cmp(&other.player)
    }
}

impl From<Player> for Entry {
    fn from(player: Player) -> Self {
        Self::new(player)
    }
}
