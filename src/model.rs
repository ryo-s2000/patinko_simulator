use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 初当たり
pub enum JackpotProbabilityDivision {
    Middle,
    LightMiddle,
    Light,
    Ama
}

// 大量に試行することを前提として、計算ロジックが複雑になる場所は期待値を用いている
pub struct Model {
    pub name: String,
    pub maker: String,
    pub price: usize,
    pub jackpot_probability: usize,
    pub jackpot_probability_division: JackpotProbabilityDivision,
    pub roll_count_within_thousand: usize, // 回転数は正規分布し大数の法則で収束する前提の値
    pub roll_per_cost: f64,
    pub st_rush_percentage: f64, // 大当たり時の時短は考慮せずに、実質突入率を計算しておく
    pub st_probability: f64,
    pub st_trials: usize,
    pub st_expected_bonus: usize, // ST時ラウンド期待値
    pub jackpot_counts: Option<Vec<HashMap<String, Vec<usize>>>>,
    pub first_win_pay_out_ball: f64, // 初当たり期待値
    pub game_count: usize,
}

impl Model {
    pub fn new(
        name: String, maker: String, price: usize, jackpot_probability: usize,
        roll_count_within_thousand: usize, st_rush_percentage: f64, st_probability: f64, st_trials: usize, st_expected_bonus: usize,
        first_win_pay_out_ball: f64,
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
            st_expected_bonus,
            jackpot_counts: None,
            first_win_pay_out_ball,
            game_count: 0,
        }
    }
}

pub fn eva15_6() -> Arc<Mutex<Model>> {
    Arc::new(
        Mutex::new(
            Model::new(
                "新世紀エヴァンゲリオン〜未来への咆哮〜".to_string(),
                "ビスティ".to_string(),
                4,
                319,
                8*2,
                70.0,
                99.4,
                163,
                1500,
                450.0,
            )
        )
    )
}
