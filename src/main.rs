extern crate rand;
use rand::Rng;

mod model;
use model::Model;
use model::eva15_6;
mod user;
use user::User;

use thousands::Separable;
use std::collections::HashMap;
use std::thread;

fn main() {
    let trials: usize = 3*4*12;

    let user_1: User = User::new("拘利無蔵".to_string(), 100);
    let user_2: User = User::new("P好太郎".to_string(), 300);

    let mut thread_handlers = vec![];

    let eva15_01 = eva15_6();

    for mut user in [user_1, user_2] {
        let selected_model = &eva15_01;
        let m = selected_model.clone();
        thread_handlers.push(
            thread::spawn(
                move || exec_and_print(&mut user, &mut m.lock().unwrap(), trials)
            )
        );
    }

    for handle in thread_handlers {
        handle.join().unwrap();
    }
}

fn exec_and_print(mut user: &mut User, mut model: &mut Model, trials: usize) {
    // 遊戯開始、初当たりするまで回す
    let mut game_counts_until_first_win = Vec::new();
    for _ in 0..trials {
        let (game_count, win) = exec_until_first_win(&mut model, user.max_traials);
        if win {
            game_counts_until_first_win.push((game_count, win));
        }
    }
    game_counts_until_first_win.sort();

    // 初当たり、STモード結果をmodelに保存
    let total_balance = exec_st_and_cal_pay_out_ball(&mut model, &game_counts_until_first_win, user.max_traials);
    user.balance = total_balance as isize;

    // 台スペック
    print_model_spec(&model);

    // 個人の結果
    println!("\n--------------{} 結果発表--------------", user.name);
    let sum = &game_counts_until_first_win.iter().map(|x| x.0).sum::<usize>();
    let average = sum / &game_counts_until_first_win.len();
    println!("平均初当たり時回転数 = {}", average);
    let sign = if user.balance > 0 {"+"} else {""};
    println!("最終収支 {}{}円", sign, user.balance.separate_with_commas());

    // 台の結果
    println!("\n--------------当たり履歴--------------");
    print_machine_results(&model);

    println!("\n");
}

fn exec_until_first_win(mut model: &mut Model, max_traials: usize) -> (usize, bool) {
    let start_game_count = model.game_count;
    let mut traial_count = start_game_count.clone();
    let mut win = false;

    loop {
        traial_count += 1;

        if rand::thread_rng().gen_range(1, model.jackpot_probability+1) == rand::thread_rng().gen_range(1, model.jackpot_probability+1) {
            model.game_count = 0;
            win = true;
            break;
        }

        if traial_count >= start_game_count+max_traials {
            model.game_count += max_traials;
            break;
        }
    }

    (traial_count, win)
}

fn exec_st_and_cal_pay_out_ball(mut model: &mut Model, game_counts_until_first_win: &Vec<(usize, bool)>, max_traials: usize) -> f64 {
    let mut total_balance: f64 = 0.0;

    // 大当たり & 確変集計
    for (game_count, win) in game_counts_until_first_win {
        let mut balance = 0.0;
        let mut pay_out_ball = 0.0;
        let mut st_game_counts = vec![];

        // 初当たり出玉
        pay_out_ball += model.first_win_pay_out_ball;

        // もし初当たりを引いていればSTに入るかの抽選が行われる
        if *win {
            if (rand::thread_rng().gen_range(0, 101) as f64) < model.st_rush_percentage {
                // 確変
                let mut st_least = model.st_trials.clone();

                loop {
                    if st_least <= 0 { break; }
                    if rand::thread_rng().gen_range(1, model.st_probability as usize) == rand::thread_rng().gen_range(1, model.st_probability as usize) {
                        let st_trials = model.st_trials.clone();
                        st_game_counts.push(st_trials - st_least);
                        st_least = model.st_trials.clone();
                        pay_out_ball += model.st_expected_bonus as f64;
                    }
                    st_least -= 1;
                }
            }
        }

        // 出玉に金額を掛ける
        balance += pay_out_ball * model.price as f64;

        // 初当たりまでに使った金額
        balance -= model.roll_per_cost * (*game_count as f64);

        let percentage =  (1 as f64-(((model.jackpot_probability-1) as f64 / model.jackpot_probability as f64).powf(*game_count as f64)))*100 as f64;
        let color_int = (game_count/model.jackpot_probability)+1;

        let sign = if balance > 0.0 {"+"} else {""};
        print_with_color(color_int, format!("{}回転({:.2}%)({}{:.2}円)(確変{}回),", game_count, percentage, sign, balance, st_game_counts.len()));

        total_balance += balance;

        if game_count < &max_traials {
            let mut jackpot_count: HashMap<String, Vec<usize>> = HashMap::new();
            jackpot_count.insert(String::from("game_count"), vec![game_count.clone()]);
            jackpot_count.insert(String::from("st_game_counts"), st_game_counts);
            jackpot_count.insert(String::from("pay_out_ball"), vec![pay_out_ball as usize]);
            match model.jackpot_counts {
                Some(ref mut x) => x.push(jackpot_count),
                None    => model.jackpot_counts = Some(vec![jackpot_count]),
            }
        }
    }

    total_balance
}

fn print_model_spec(model: &Model) {
    println!("\n{}", model.name);
    println!("メーカー名 {}", model.maker);
    println!("大当り確率 1/{}(通常時) 1/{}(高確率時)", model.jackpot_probability, model.st_probability);
    println!("回転数 1000円あたり{}回転", model.roll_count_within_thousand);
    println!("1回転あたり{:.2}円", model.roll_per_cost);
}

fn print_with_color(color_int: usize, text: String) {
    print!("\x1b[{}m{}\x1b[m", 30+color_int, text);
}

fn print_machine_results(model: &Model) {
    print_model_spec(&model);
    let jackpot_counts = model.jackpot_counts.as_ref().unwrap();
    for _ in 0..5 {
        let jackpot_count = &jackpot_counts[
            rand::thread_rng().gen_range(1, &jackpot_counts.len()-1 as usize)
        ];
        let st_game_counts = jackpot_count.get("st_game_counts").unwrap().len();
        println!("{}|{}回転|{}連|{}発",
            if &st_game_counts > &1 { "確変" } else { "通常" },
            jackpot_count.get("game_count").unwrap()[0],
            &st_game_counts,
            jackpot_count.get("pay_out_ball").unwrap()[0],
        );
    }
}
