use serde::Deserialize;
use serde::Serialize;
use sysinfo::Components;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    pub name: String,
    pub temperature: f32,
}

impl TemperatureInfo {
    pub fn get_temperatures() -> Vec<Self> {
        let components = Components::new_with_refreshed_list();
        let mut temperature_info_vec = vec![];
        for component in &components {
            let (name, temperature) = (component.label().to_owned(), component.temperature());
            let temp_info = TemperatureInfo { name, temperature };
            temperature_info_vec.push(temp_info);
        }
        temperature_info_vec
    }
}