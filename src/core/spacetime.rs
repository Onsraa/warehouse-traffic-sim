use bevy::prelude::*;
use rustc_hash::FxHashMap;
use super::GridPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpaceTimeKey {
    pub pos: GridPos,
    pub tick: u64,
}

impl SpaceTimeKey {
    #[inline]
    pub const fn new(pos: GridPos, tick: u64) -> Self {
        Self { pos, tick }
    }
}

#[derive(Resource, Default)]
pub struct SpaceTimeTable {
    reservations: FxHashMap<SpaceTimeKey, Entity>,
    current_tick: u64,
}

impl SpaceTimeTable {
    pub fn reserve(&mut self, pos: GridPos, tick: u64, entity: Entity) -> bool {
        let key = SpaceTimeKey::new(pos, tick);
        if let Some(&existing) = self.reservations.get(&key) {
            if existing != entity {
                return false;
            }
        }
        self.reservations.insert(key, entity);
        true
    }

    pub fn reserve_path(&mut self, path: &[(GridPos, u64)], entity: Entity) -> bool {
        // Vérifie d'abord
        for &(pos, tick) in path {
            let key = SpaceTimeKey::new(pos, tick);
            if let Some(&occupant) = self.reservations.get(&key) {
                if occupant != entity {
                    return false;
                }
            }
        }
        // Puis réserve
        for &(pos, tick) in path {
            self.reservations.insert(SpaceTimeKey::new(pos, tick), entity);
        }
        true
    }

    #[inline]
    pub fn is_free(&self, pos: GridPos, tick: u64, exclude: Option<Entity>) -> bool {
        match self.reservations.get(&SpaceTimeKey::new(pos, tick)) {
            None => true,
            Some(&e) => exclude.is_some_and(|ex| ex == e),
        }
    }

    pub fn is_edge_free(&self, from: GridPos, to: GridPos, tick: u64, exclude: Option<Entity>) -> bool {
        // Vérifie qu'aucun robot ne fait le mouvement inverse (swap)
        let key_to_at_tick = SpaceTimeKey::new(to, tick);
        let key_from_at_next = SpaceTimeKey::new(from, tick + 1);

        let check = |key: SpaceTimeKey| -> bool {
            match self.reservations.get(&key) {
                None => true,
                Some(&e) => exclude.is_some_and(|ex| ex == e),
            }
        };

        check(key_to_at_tick) && check(key_from_at_next)
    }

    pub fn clear_entity(&mut self, entity: Entity) {
        self.reservations.retain(|_, &mut e| e != entity);
    }

    /// Efface les réservations d'une entité sauf sa position actuelle
    pub fn clear_entity_except_pos(&mut self, entity: Entity, current_pos: GridPos, current_tick: u64) {
        self.reservations.retain(|key, &mut e| {
            if e != entity {
                return true;
            }
            // Garde les réservations de la position actuelle pour les prochains ticks
            key.pos == current_pos && key.tick >= current_tick && key.tick < current_tick + 3
        });
    }

    pub fn cleanup(&mut self, current_tick: u64) {
        self.reservations.retain(|key, _| key.tick >= current_tick.saturating_sub(1));
        self.current_tick = current_tick;
    }

    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
    }
}