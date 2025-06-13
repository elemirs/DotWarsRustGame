use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// GAME STATE
// ============================================================================

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    MainMenu,
    WorldMap,
    Battle,
    Diplomacy,
}

// ============================================================================
// CORE IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProvinceId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechnologyId(pub Uuid);

impl Default for FactionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ProvinceId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UnitId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TechnologyId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

// ============================================================================
// CORE COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub color: Color,
}

// ============================================================================
// RESOURCES SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub gold: i32,
    pub food: i32,
    pub materials: i32,
    pub manpower: i32,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            food: 500,
            materials: 300,
            manpower: 100,
        }
    }
    
    pub fn can_afford(&self, cost: &Resources) -> bool {
        self.gold >= cost.gold
            && self.food >= cost.food
            && self.materials >= cost.materials
            && self.manpower >= cost.manpower
    }
    
    pub fn subtract(&mut self, cost: &Resources) -> bool {
        if self.can_afford(cost) {
            self.gold -= cost.gold;
            self.food -= cost.food;
            self.materials -= cost.materials;
            self.manpower -= cost.manpower;
            true
        } else {
            false
        }
    }
    
    pub fn add(&mut self, income: &Resources) {
        self.gold += income.gold;
        self.food += income.food;
        self.materials += income.materials;
        self.manpower += income.manpower;
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CORE TRAITS
// ============================================================================

pub trait Updatable {
    fn update(&mut self, delta_time: f32);
}

pub trait Serializable {
    fn serialize(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn deserialize(data: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
