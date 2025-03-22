// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/

pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    let mut supplies = supplies.into_iter().collect::<HashSet<_>>();
    let mut recipes = recipes
        .iter()
        .zip(ingredients.into_iter())
        .map(|(recipe, ingredients)| (recipe, ingredients.into_iter().collect::<HashSet<_>>()))
        .collect::<HashMap<_, _>>();

    let mut result: Vec<String> = vec![];

    loop {
        let old_len = recipes.len();
        recipes = recipes
            .into_iter()
            .filter(|recipe| {
                if supplies.is_superset(&recipe.1) {
                    result.push(recipe.0.to_string());
                    supplies.insert(recipe.0.to_string());
                    false
                } else {
                    true
                }
            })
            .collect();
        let new_len = recipes.len();
        if new_len == old_len {
            break;
        }
    }

    result
}
