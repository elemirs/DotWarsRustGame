use bevy::prelude::*;
use dot_wars_core::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// WORLD MAP STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub id: ProvinceId,
    pub name: String,
    pub owner: Option<FactionId>,
    pub position: Position,
    pub population: u32,
    pub resources: Resources,
    pub buildings: Vec<Building>,
    pub adjacent_provinces: Vec<ProvinceId>,
    pub terrain_type: TerrainType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Plains,
    Forest,
    Mountains,
    Desert,
    Swamp,
    Coast,
}

impl TerrainType {
    pub fn movement_cost(&self) -> f32 {
        match self {
            TerrainType::Plains => 1.0,
            TerrainType::Forest => 1.5,
            TerrainType::Mountains => 2.0,
            TerrainType::Desert => 1.8,
            TerrainType::Swamp => 2.5,
            TerrainType::Coast => 1.0,
        }
    }
    
    pub fn defense_bonus(&self) -> f32 {
        match self {
            TerrainType::Plains => 0.0,
            TerrainType::Forest => 0.2,
            TerrainType::Mountains => 0.5,
            TerrainType::Desert => 0.1,
            TerrainType::Swamp => 0.3,
            TerrainType::Coast => 0.0,
        }
    }
}

// ============================================================================
// BUILDINGS SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
    pub construction_progress: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    City,
    Farm,
    Mine,
    Barracks,
    Workshop,
    Fort,
    Port,
    Temple,
}

impl BuildingType {
    pub fn construction_cost(&self, level: u32) -> Resources {
        let base_cost = match self {
            BuildingType::City => Resources { gold: 500, materials: 300, ..Default::default() },
            BuildingType::Farm => Resources { gold: 200, materials: 100, ..Default::default() },
            BuildingType::Mine => Resources { gold: 300, materials: 200, ..Default::default() },
            BuildingType::Barracks => Resources { gold: 400, materials: 250, ..Default::default() },
            BuildingType::Workshop => Resources { gold: 350, materials: 400, ..Default::default() },
            BuildingType::Fort => Resources { gold: 600, materials: 500, ..Default::default() },
            BuildingType::Port => Resources { gold: 800, materials: 400, ..Default::default() },
            BuildingType::Temple => Resources { gold: 450, materials: 200, ..Default::default() },
        };
        
        // Cost increases with level
        Resources {
            gold: base_cost.gold * level as i32,
            food: base_cost.food * level as i32,
            materials: base_cost.materials * level as i32,
            manpower: base_cost.manpower * level as i32,
        }
    }
    
    pub fn resource_income(&self, level: u32) -> Resources {
        let base_income = match self {
            BuildingType::City => Resources { gold: 50, ..Default::default() },
            BuildingType::Farm => Resources { food: 100, ..Default::default() },
            BuildingType::Mine => Resources { materials: 80, ..Default::default() },
            BuildingType::Barracks => Resources { manpower: 20, ..Default::default() },
            BuildingType::Workshop => Resources { gold: 30, materials: 20, ..Default::default() },
            BuildingType::Fort => Resources::default(), // Forts don't generate resources
            BuildingType::Port => Resources { gold: 80, ..Default::default() },
            BuildingType::Temple => Resources { gold: 20, ..Default::default() },
        };
        
        Resources {
            gold: base_income.gold * level as i32,
            food: base_income.food * level as i32,
            materials: base_income.materials * level as i32,
            manpower: base_income.manpower * level as i32,
        }
    }
}

// ============================================================================
// WORLD MAP RESOURCE
// ============================================================================

#[derive(Resource, Debug)]
pub struct WorldMap {
    pub provinces: HashMap<ProvinceId, Province>,
    pub faction_territories: HashMap<FactionId, Vec<ProvinceId>>,
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            provinces: HashMap::new(),
            faction_territories: HashMap::new(),
        }
    }
    
    pub fn add_province(&mut self, province: Province) {
        if let Some(owner) = province.owner {
            self.faction_territories
                .entry(owner)
                .or_insert_with(Vec::new)
                .push(province.id);
        }
        self.provinces.insert(province.id, province);
    }
    
    pub fn get_province(&self, id: ProvinceId) -> Option<&Province> {
        self.provinces.get(&id)
    }
    
    pub fn get_province_mut(&mut self, id: ProvinceId) -> Option<&mut Province> {
        self.provinces.get_mut(&id)
    }
    
    pub fn get_faction_provinces(&self, faction_id: FactionId) -> Vec<&Province> {
        self.faction_territories
            .get(&faction_id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|&id| self.provinces.get(&id))
            .collect()
    }
    
    pub fn calculate_faction_income(&self, faction_id: FactionId) -> Resources {
        let mut total_income = Resources::default();
        
        for province in self.get_faction_provinces(faction_id) {
            for building in &province.buildings {
                let income = building.building_type.resource_income(building.level);
                total_income.add(&income);
            }
        }
        
        total_income
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// WORLD GENERATION
// ============================================================================

pub struct WorldGenerator;

impl WorldGenerator {
    pub fn generate_world(width: u32, height: u32, num_provinces: u32) -> WorldMap {
        let mut world_map = WorldMap::new();
        
        // Generate provinces in a grid-like pattern with some randomness
        for i in 0..num_provinces {
            let x = (i % width) as f32 * 100.0;
            let y = (i / width) as f32 * 100.0;
            
            let province = Province {
                id: ProvinceId::default(),
                name: format!("Province {}", i + 1),
                owner: None,
                position: Position::new(x, y),
                population: 1000 + (i * 500),
                resources: Resources::new(),
                buildings: vec![
                    Building {
                        building_type: BuildingType::City,
                        level: 1,
                        construction_progress: 1.0,
                    }
                ],
                adjacent_provinces: Vec::new(),
                terrain_type: Self::random_terrain_type(i),
            };
            
            world_map.add_province(province);
        }
        
        world_map
    }
    
    fn random_terrain_type(seed: u32) -> TerrainType {
        match seed % 6 {
            0 => TerrainType::Plains,
            1 => TerrainType::Forest,
            2 => TerrainType::Mountains,
            3 => TerrainType::Desert,
            4 => TerrainType::Swamp,
            5 => TerrainType::Coast,
            _ => TerrainType::Plains,
        }
    }
}
