// 初当たり

pub enum JackpotProbabilityDivision {
    Middle,
    LightMiddle,
    Light,
    Ama
}

pub struct Model {
    pub name: String,
    pub maker: String,
    pub price: usize,
    pub jackpot_probability: usize,
    pub jackpot_probability_division: JackpotProbabilityDivision,
    pub jackpot_pay_out_average: usize, // TODO ST mode
    pub roll_count_within_thousand: usize,
    pub roll_per_cost: f64,
}

impl Model {
    pub fn new(name: String, maker: String, price: usize, jackpot_probability: usize, jackpot_pay_out_average: usize, roll_count_within_thousand: usize) -> Self {
        let jackpot_probability_division= match jackpot_probability {
            251..=1000 => JackpotProbabilityDivision::Middle,
            151..=250 => JackpotProbabilityDivision::LightMiddle,
            100..=150 => JackpotProbabilityDivision::Light,
            _ => JackpotProbabilityDivision::Ama
        };

        Model {
            name,
            maker,
            price,
            jackpot_probability,
            jackpot_probability_division,
            jackpot_pay_out_average,
            roll_count_within_thousand,
            roll_per_cost: 1000.0/roll_count_within_thousand as f64,
        }
    }
}
