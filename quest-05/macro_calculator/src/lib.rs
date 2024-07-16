use json;

pub struct Food {
    //expected public fields
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let (mut total_calories, mut total_fats, mut total_carbs, mut total_proteins) =
        (0.0, 0.0, 0.0, 0.0);

    for food in foods {
        let calories = food.calories[1].replace("kcal", "").parse::<f64>().unwrap();
        total_calories += calories * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    json::object! {
        cals: (total_calories * 100.0 ).round() / 100.0,
        carbs: (total_carbs * 100.0 ).round() / 100.0,
        proteins: (total_proteins * 100.0 ).round() / 100.0,
        fats: (total_fats * 100.0 ).round() / 100.0,
    }
}
