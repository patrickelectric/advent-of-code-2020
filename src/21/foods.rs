use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Foods {
    pub allergen_names: HashMap<String, Vec<String>>,
    pub ingredients: Vec<String>,
}

impl Foods {
    pub fn new() -> Self {
        return Self {
            allergen_names: HashMap::new(),
            ingredients: vec![],
        };
    }

    pub fn process(&mut self, line: &str) {
        let re = Regex::new(r"\w+").unwrap();
        let lines = re
            .captures_iter(line)
            .filter(|capture| capture.get(0).is_some())
            .map(|capture| capture.get(0).unwrap().as_str().to_string())
            .collect::<Vec<String>>();
        let mut lines_iter = lines.split(|string| string == "contains");
        let ingredients: Vec<String> = lines_iter
            .next()
            .unwrap()
            .iter()
            .map(|ingredient| ingredient.clone())
            .collect();
        self.ingredients.append(&mut ingredients.clone());
        let allergens = lines_iter.next().unwrap();
        for allergen in allergens {
            match self.allergen_names.get_mut(allergen) {
                Some(content) => {
                    let ingredients = content
                        .iter()
                        .filter(|ingredient| ingredients.contains(ingredient))
                        .clone();
                    let ingredients: Vec<String> = ingredients.map(Into::into).collect();
                    self.allergen_names
                        .insert(allergen.to_string(), ingredients.clone());
                }
                None => {
                    self.allergen_names
                        .insert(allergen.to_string(), ingredients.clone());
                }
            }
        }
    }

    pub fn pos_processing(&mut self) {
        let keys: Vec<String> = self.allergen_names.keys().map(Into::into).collect();
        while self
            .allergen_names
            .values()
            .filter(|values| values.len() != 1)
            .count()
            != 0
        {
            for key1 in keys.iter() {
                if self.allergen_names[key1].len() != 1 {
                    continue;
                }
                for key2 in keys.iter() {
                    if self.allergen_names[key2].len() < 2 {
                        continue;
                    }
                    if key1 == key2 {
                        continue;
                    }

                    let ingredients: Vec<String> = self.allergen_names[key2]
                        .iter()
                        .filter(|ingredient| !self.allergen_names[key1].contains(ingredient))
                        .map(Into::into)
                        .collect();
                    self.allergen_names.insert(key2.clone(), ingredients);
                }
            }
        }

        self.ingredients.sort();
    }

    pub fn know_ingredients(&self) -> Vec<String> {
        return self
            .allergen_names
            .values()
            .map(|vector| vector.first().unwrap())
            .map(Into::into)
            .collect();
    }

    pub fn unknown_ingredients(&self) -> Vec<String> {
        let know_ingredients = self.know_ingredients();
        return self
            .ingredients
            .iter()
            .filter(|ingredient| !know_ingredients.contains(ingredient))
            .map(Into::into)
            .collect();
    }

    pub fn dangerous_list(&self) -> String {
        let mut keys: Vec<String> = self.allergen_names.keys().map(Into::into).collect();
        keys.sort();
        return keys
            .iter()
            .map(|key| self.allergen_names[key].first().unwrap())
            .map(Into::into)
            .collect::<Vec<String>>()
            .join(",");
    }
}
