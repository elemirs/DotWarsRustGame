use bevy::prelude::*;
use dot_wars_core::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// UNIT SYSTEM
// ============================================================================

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub unit_type: UnitType,
    pub count: u32,
    pub max_count: u32,
    pub morale: f32, // 0.0 to 100.0
    pub experience: u32,
    pub formation: Formation,
    pub faction: FactionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitType {
    Infantry,
    Cavalry,
    Archers,
    Artillery,
    Special(String), // For unique units
}

impl UnitType {
    pub fn stats(&self) -> UnitStats {
        match self {
            UnitType::Infantry => UnitStats {
                attack: 15,
                defense: 12,
                health: 100,
                speed: 50.0,
                range: 1.0,
                cost: Resources { gold: 100, manpower: 10, ..Default::default() },
            },
            UnitType::Cavalry => UnitStats {
                attack: 20,
                defense: 8,
                health: 120,
                speed: 100.0,
                range: 1.0,
                cost: Resources { gold: 200, manpower: 15, ..Default::default() },
            },
            UnitType::Archers => UnitStats {
                attack: 18,
                defense: 6,
                health: 80,
                speed: 40.0,
                range: 150.0,
                cost: Resources { gold: 120, manpower: 12, ..Default::default() },
            },
            UnitType::Artillery => UnitStats {
                attack: 35,
                defense: 5,
                health: 60,
                speed: 20.0,
                range: 300.0,
                cost: Resources { gold: 500, materials: 100, manpower: 8, ..Default::default() },
            },
            UnitType::Special(_) => UnitStats {
                attack: 25,
                defense: 15,
                health: 150,
                speed: 60.0,
                range: 50.0,
                cost: Resources { gold: 800, manpower: 25, ..Default::default() },
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitStats {
    pub attack: u32,
    pub defense: u32,
    pub health: u32,
    pub speed: f32,
    pub range: f32,
    pub cost: Resources,
}

// ============================================================================
// FORMATION SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formation {
    Line,
    Column,
    Square,
    Wedge,
    Skirmish,
}

impl Formation {
    pub fn get_modifiers(&self) -> FormationModifiers {
        match self {
            Formation::Line => FormationModifiers {
                attack_modifier: 1.0,
                defense_modifier: 1.0,
                speed_modifier: 1.0,
                morale_modifier: 1.0,
            },
            Formation::Column => FormationModifiers {
                attack_modifier: 0.8,
                defense_modifier: 0.7,
                speed_modifier: 1.3,
                morale_modifier: 1.1,
            },
            Formation::Square => FormationModifiers {
                attack_modifier: 0.6,
                defense_modifier: 1.5,
                speed_modifier: 0.5,
                morale_modifier: 1.2,
            },
            Formation::Wedge => FormationModifiers {
                attack_modifier: 1.3,
                defense_modifier: 0.8,
                speed_modifier: 1.1,
                morale_modifier: 0.9,
            },
            Formation::Skirmish => FormationModifiers {
                attack_modifier: 0.9,
                defense_modifier: 1.2,
                speed_modifier: 1.2,
                morale_modifier: 0.8,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormationModifiers {
    pub attack_modifier: f32,
    pub defense_modifier: f32,
    pub speed_modifier: f32,
    pub morale_modifier: f32,
}

// ============================================================================
// BATTLE SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battle {
    pub id: String,
    pub attacker: FactionId,
    pub defender: FactionId,
    pub attacker_units: Vec<UnitId>,
    pub defender_units: Vec<UnitId>,
    pub battlefield: Battlefield,
    pub phase: BattlePhase,
    pub turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattlePhase {
    Deployment,
    Combat,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battlefield {
    pub width: f32,
    pub height: f32,
    pub terrain_effects: Vec<TerrainEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainEffect {
    pub position: Position,
    pub radius: f32,
    pub effect_type: TerrainEffectType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainEffectType {
    HighGround { defense_bonus: f32 },
    Forest { concealment: f32 },
    River { movement_penalty: f32 },
    Fortification { defense_bonus: f32, attack_bonus: f32 },
}

// ============================================================================
// COMBAT SYSTEM
// ============================================================================

#[derive(Component)]
pub struct CombatStats {
    pub attack: f32,
    pub defense: f32,
    pub accuracy: f32,
    pub evasion: f32,
}

pub struct CombatSystem;

impl CombatSystem {
    pub fn calculate_damage(
        attacker: &Unit,
        attacker_stats: &CombatStats,
        defender: &Unit,
        defender_stats: &CombatStats,
    ) -> u32 {
        let base_damage = attacker_stats.attack;
        let defense_reduction = defender_stats.defense;
        
        // Apply formation modifiers
        let formation_mods = attacker.formation.get_modifiers();
        let modified_attack = base_damage * formation_mods.attack_modifier;
        
        let defender_formation_mods = defender.formation.get_modifiers();
        let modified_defense = defense_reduction * defender_formation_mods.defense_modifier;
        
        // Calculate final damage
        let final_damage = (modified_attack - modified_defense).max(1.0);
        
        // Apply morale effect
        let morale_factor = attacker.morale / 100.0;
        (final_damage * morale_factor) as u32
    }
    
    pub fn apply_casualties(unit: &mut Unit, casualties: u32) {
        unit.count = unit.count.saturating_sub(casualties);
        
        // Morale loss based on casualties
        let casualty_ratio = casualties as f32 / unit.max_count as f32;
        let morale_loss = casualty_ratio * 20.0; // Lose up to 20 morale
        unit.morale = (unit.morale - morale_loss).max(0.0);
    }
    
    pub fn check_rout(unit: &Unit) -> bool {
        // Unit routs if morale is too low or too few soldiers remain
        unit.morale < 20.0 || (unit.count as f32 / unit.max_count as f32) < 0.1
    }
}

// ============================================================================
// BATTLE AI
// ============================================================================

pub struct BattleAI;

impl BattleAI {
    pub fn choose_formation(unit_type: &UnitType, enemy_units: &[&Unit]) -> Formation {
        match unit_type {
            UnitType::Infantry => {
                // If facing cavalry, use square formation
                if enemy_units.iter().any(|u| matches!(u.unit_type, UnitType::Cavalry)) {
                    Formation::Square
                } else {
                    Formation::Line
                }
            },
            UnitType::Cavalry => Formation::Wedge,
            UnitType::Archers => Formation::Skirmish,
            UnitType::Artillery => Formation::Line,
            UnitType::Special(_) => Formation::Line,
        }
    }
    
    pub fn choose_target(attacking_unit: &Unit, enemy_units: &[&Unit]) -> Option<UnitId> {
        // Simple AI: target weakest enemy first
        enemy_units
            .iter()
            .min_by_key(|unit| unit.count)
            .map(|unit| unit.id)
    }
}
