use bevy::prelude::*;
use rustc_hash::FxHashMap;
use super::GridPos;

/// Clé espace-temps: position + tick
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

/// Table de réservations espace-temps
#[derive(Resource, Default)]
pub struct SpaceTimeTable {
    reservations: FxHashMap<SpaceTimeKey, Entity>,
    current_tick: u64,
}

impl SpaceTimeTable {
    /// Réserve une cellule pour un tick donné
    pub fn reserve(&mut self, pos: GridPos, tick: u64, entity: Entity) -> bool {
        let key = SpaceTimeKey::new(pos, tick);
        if self.reservations.contains_key(&key) {
            return false;
        }
        self.reservations.insert(key, entity);
        true
    }

    /// Réserve un chemin complet (séquence de positions)
    pub fn reserve_path(&mut self, path: &[(GridPos, u64)], entity: Entity) -> bool {
        // Vérifie d'abord que tout est libre
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

    /// Vérifie si une cellule est libre à un tick donné
    #[inline]
    pub fn is_free(&self, pos: GridPos, tick: u64, exclude: Option<Entity>) -> bool {
        match self.reservations.get(&SpaceTimeKey::new(pos, tick)) {
            None => true,
            Some(&e) => exclude.is_some_and(|ex| ex == e),
        }
    }

    /// Vérifie l'absence de conflit d'arête (swap conflict)
    pub fn is_edge_free(&self, from: GridPos, to: GridPos, tick: u64, exclude: Option<Entity>) -> bool {
        // Vérifie que personne ne fait le mouvement inverse au même moment
        let key_from = SpaceTimeKey::new(to, tick);
        let key_to = SpaceTimeKey::new(from, tick + 1);

        let from_ok = match self.reservations.get(&key_from) {
            None => true,
            Some(&e) => exclude.is_some_and(|ex| ex == e),
        };

        let to_ok = match self.reservations.get(&key_to) {
            None => true,
            Some(&e) => exclude.is_some_and(|ex| ex == e),
        };

        from_ok && to_ok
    }

    /// Libère toutes les réservations d'une entité
    pub fn clear_entity(&mut self, entity: Entity) {
        self.reservations.retain(|_, &mut e| e != entity);
    }

    /// Nettoie les réservations expirées
    pub fn cleanup(&mut self, current_tick: u64) {
        self.reservations.retain(|key, _| key.tick >= current_tick);
        self.current_tick = current_tick;
    }

    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
    }
}