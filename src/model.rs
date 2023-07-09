use std::collections::HashMap;

// 初当たり
pub enum JackpotProbabilityDivision {
    Middle,
    LightMiddle,
    Light,
    Ama
}

// 時短は考慮せずに、実質突入率を計算すること
// TODO ラウンドごとの確率変更、出玉変更設定
// TODO 回転数は正規分布する
pub struct Model {
    pub name: String,
    pub maker: String,
    pub price: usize,
    pub jackpot_probability: usize,
    pub jackpot_probability_division: JackpotProbabilityDivision,
    pub roll_count_within_thousand: usize,
    pub roll_per_cost: f64,
    pub st_rush_percentage: f64,
    pub st_probability: f64,
    pub st_trials: usize,
    pub st_bonus: usize,
    pub jackpot_counts: Option<Vec<HashMap<String, Vec<usize>>>>
}

impl Model {
    pub fn new(
        name: String, maker: String, price: usize, jackpot_probability: usize,
        roll_count_within_thousand: usize, st_rush_percentage: f64, st_probability: f64, st_trials: usize, st_bonus: usize,
    ) -> Self {
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
            roll_count_within_thousand,
            roll_per_cost: 1000.0/roll_count_within_thousand as f64,
            st_rush_percentage,
            st_probability,
            st_trials,
            st_bonus,
            jackpot_counts: None,
        }
    }
}
// TODO 遊タイム
