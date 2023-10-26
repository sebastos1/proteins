use std::collections::HashMap;
use indexmap::IndexMap;

pub const RDI_PATH: &str = "data/rdis.json";
pub const FOOD_PATH: &str = "data/food.json";
pub const API_PATH: &str = "data/api_link.txt";
pub const CUSTOM_PATH: &str = "data/custom.json";
pub const DICTIONARY_PATH: &str = "data/dictionary.json";

pub fn get_active_columns() -> Vec<String> {
    return vec![
        "kcal".to_string(), 
        "protein".to_string(), 
        "carb".to_string(), 
        "fat".to_string()
    ];
}

pub fn load_dictionary<'a>() -> IndexMap<String, HashMap<String, String>> {
    serde_json::from_str(&std::fs::read_to_string(DICTIONARY_PATH).unwrap()).unwrap()
}

pub fn convert_id<'a>() -> Vec<(&'a str, &'a str)> {
    return vec![("Energi1", "kJ"), ("Energi2", "kcal"), ("Netto", "edible_part"), ("Vann", "water"), ("Fett", "fat"), ("Mettet", "saturated_fat"), ("C12:0Laurinsyre", "lauric_acid"), ("C14:0Myristinsyre", "myristic_acid"), ("C16:0Palmitinsyre", "palmitic_acid"), ("C18:0Stearinsyre", "stearic_acid"), ("Trans", "trans"), ("Enumet", "monounsat_fat"), ("C16:1", "C16:1"), ("C18:1", "C18:1"), ("Flerum", "polyunsat_fat"), ("C18:2n-6Linolsyre", "poly_1"), ("C18:3n-3AlfaLinolensyre", "poly_2"), ("C20:3n-3Eikosatriensyre", "poly_3"), ("C20:3n-6DihomoGammaLinolensyre", "poly_4"), ("C20:4n-3Eikosatetraensyre", "poly_5"), ("C20:4n-6Arakidonsyre", "poly_6"), ("C20:5n-3Eikosapentaensyre", "poly_7"), ("C22:5n-3Dokosapentaensyre", "poly_8"), ("C22:6n-3Dokosaheksaensyre", "poly_9"), ("Omega-3", "omega3"), ("Omega-6", "omega6"), ("Kolest", "cholesterol"), ("Karbo", "carb"), ("Stivel", "starch"), ("Mono+Di", "mono+di"), ("Sukker", "sugar"), ("Fiber", "fiber"), ("Protein", "protein"), ("NaCl", "salt"), ("Alko", "alcohol"), ("Vit A", "vit_a"), ("Retinol", "retinol"), ("B-karo", "beta_carotene"), ("Vit D", "vit_d"), ("Vit E", "vit_e"), ("Vit B1", "vit_b1"), ("Vit B2", "vit_b2"), ("Niacin", "vit_b3"), ("Vit B6", "vit_b6"), ("Folat", "folate"), ("Vit B12", "vit_b12"), ("Vit C", "vit_c"), ("Ca", "ca"), ("Fe", "fe"), ("Na", "na"), ("K", "k"), ("Mg", "mg"), ("Zn", "zn"), ("Se", "se"), ("Cu", "cu"), ("P", "p"), ("I", "i")];
}

pub fn load_rdis() -> HashMap<String, HashMap<String, String>> {
    serde_json::from_str(&std::fs::read_to_string(RDI_PATH).unwrap()).unwrap()
}