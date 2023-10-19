pub fn get_active_columns() -> Vec<String> {
    return vec![
        "kcal".to_string(), 
        "Protein".to_string(), 
        "Karbohydrater".to_string(), 
        "Fett".to_string()
    ];
}

// add language support
pub fn get_nutrient_order<'a>() -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    for (_, unit, norwegian, _) in dictionary() {
        vec.push((norwegian, unit))
    }
    return vec
}

pub fn get_long_names<'a>(language: &str) -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    match language {
        "no" => {
            for (code, _, norwegian, _) in dictionary() {
                vec.push((norwegian, code))
            }
        },
        _ => {
            for (code, _, _, english) in dictionary() {
                vec.push((english, code))
            }
        }
    }
    return vec
}

// pub fn reorder_dictionary<'a>() {
//     for (new, unit, old, english) in dictionary() {
//         println!("(\"{}\", \"{}\", \"{}\", \"{}\"),", old, unit, new, english);
//     }
// }

pub fn dictionary<'a>() -> Vec<(&'a str, &'a str, &'a str, &'a str)> {
    return vec![
        // original, unit, norwegian, english, RDI
        ("Energi1", "kJ", "kJ", "kJ"),  
        ("Energi2", "kcal", "kcal", "kcal"),
        ("Netto", "g", "Spiselig andel", "Edible part"),  
        ("Vann", "g", "Vann", "Water"),  
        ("Fett", "g", "Fett", "Fat"),  
            ("Mettet", "g", "Mettet fett", "Saturated fat"),  
                ("C12:0Laurinsyre", "g", "Laurinsyre (C12:0)", "todo"),
                ("C14:0Myristinsyre", "g", "Myristinsyre (C14:0)", "todo"),
                ("C16:0Palmitinsyre", "g", "Palmitinsyre (C16:0)", "todo"),
                ("C18:0Stearinsyre", "g", "Stearinsyre (C18:0)", "todo"),
            ("Trans", "g", "Transfett", "Trans fat"),
            ("Enumet", "g", "Enumettet fett", "Monounsaturated fat"),  
                ("C16:1", "g", "C16:1 sum", "C16:1 sum"),
                ("C18:1", "g", "C18:1 sum", "C18:1 sum"),
            ("Flerum", "g", "Flerumettet fett", "Polyunsaturated fat"),
                ("C18:2n-6Linolsyre", "g", "Linolsyre (C18:2n-6)", "todo"),
                ("C18:3n-3AlfaLinolensyre", "g", "Alfalinolensyre (C18:3n-3)", "todo"),
                ("C20:3n-3Eikosatriensyre", "g", "Eikosatriensyre (C20:3n-3)", "todo"),
                ("C20:3n-6DihomoGammaLinolensyre", "g", "Dihomogammalinolensyre (C20:3n-6)", "todo"),
                ("C20:4n-3Eikosatetraensyre", "g", "Eikosatetraensyre (C20:4n-3)", "todo"),
                ("C20:4n-6Arakidonsyre", "g", "Arakidonsyre (C20:4n-6)", "todo"),
                ("C20:5n-3Eikosapentaensyre", "g", "Eikosapentaensyre (C20:5n-3)", "todo"),
                ("C22:5n-3Dokosapentaensyre", "g", "Dokosapentaensyre (C22:5n-3)", "todo"),
                ("C22:6n-3Dokosaheksaensyre", "g", "Dokosaheksaensyre (C22:6n-3)", "todo"),
                ("Omega-3", "g", "Omega-3", "Omega-3"),  
                ("Omega-6", "g", "Omega-6", "Omega-6"),  
            ("Kolest", "mg", "Kolesterol", "Cholesterol"),  
        ("Karbo", "g", "Karbohydrater", "Carbohydrates"),  
            ("Stivel", "g", "Stivelse", "Starch"),  
            ("Mono+Di", "g", "Mono- & disakkarider", "Mono- & disaccharides"),  
            ("Sukker", "g", "Sukkerarter", "Sugars"), 
        ("Fiber", "g", "Fiber", "Fiber"),  
        ("Protein", "g", "Protein", "Protein"),  
        ("NaCl", "g", "Salt", "Salt"),  
        ("Alko", "g", "Alkohol", "Alcohol"),
        ("Vit A", "µg", "Vitamin A", "Vit A"),  
            ("Retinol", "µg", "Retinol", "Retinol"),  
            ("B-karo", "µg", "Betakaroten", "Beta-carotene"), 
        ("Vit D", "µg", "Vit D", "Vit D"),  
        ("Vit E", "mg", "Vit E", "Vit E"),  
        ("Vit B1", "mg", "Thiamin (Vit B1)", " Thiamine (Vit B1)"),  
        ("Vit B2", "mg", "Riboflavin (Vit B2)", "Riboflavin (Vit B2)"),  
        ("Niacin", "mg", "Niacin (B3)", "Niacin (B3)"),  
        ("Vit B6", "mg", "Vit B6", "Vit 6"),  
        ("Folat", "µg", "Folat (B9)", "Folate (B9)"),  
        ("Vit B12", "µg", "Vit B12", "Vit B12"),  
        ("Vit C", "mg", "Vit C", "Vit C"),  
        ("Ca", "mg", "Kalsium", "Calcium"),  
        ("Fe", "mg", "Jern", "Iron"),  
        ("Na", "mg", "Natrium", "Sodium"),  
        ("K", "mg", "Kalium", "Potassium"),  
        ("Mg", "mg", "Magnesium", "Magnesium"),  
        ("Zn", "mg", "Sink", "Zinc"),  
        ("Se", "µg", "Selenium", "Selenium"),  
        ("Cu", "mg", "Kobber", "Copper"),  
        ("P", "mg", "Fosfor", "Phosphorus"), 
        ("I", "µg", "Jod", "Iodine"),   
    ];
}
