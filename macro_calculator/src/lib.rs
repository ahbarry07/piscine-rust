use json::{self, object};
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}


pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {

    let mut cals_sum: f64 = 0.0;
    let mut carbs_sum: f64 = 0.0;
    let mut prot_sum: f64 = 0.0;
    let mut fats_sum: f64 = 0.0;

    for food in foods{
        cals_sum += food.calories[1].to_string().replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs_sum += food.carbs * food.nbr_of_portions;
        prot_sum += food.proteins * food.nbr_of_portions;
        fats_sum += food.fats * food.nbr_of_portions;
    }

    object! {
        "cals": format_value(cals_sum),
        "carbs": format_value(carbs_sum),
        "proteins": format_value(prot_sum),
        "fats": format_value(fats_sum)
    }
}

fn format_value(v: f64) -> f64{
    if v.to_string().ends_with("0"){
        return format!("{:.1}", v).parse::<f64>().unwrap() ;
    }
    format!("{:.2}", v).parse::<f64>().unwrap()
}