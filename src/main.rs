extern crate rand;
use rand::Rng;

mod model;
use model::Model;

mod user;
use user::User;

use thousands::Separable;
use std::collections::HashMap;

static MAX_TRAIAL: usize = 6000;

fn main() {
    let trials = 3*4*12;

    let user_1: User = User::new("拘利無蔵".to_string());
    let user_2: User = User::new("P好太郎".to_string());

    let code_6 = Model::new(
        "P コードギアス 反逆のルルーシュ Rebellion to Re;surrection".to_string(),
        "ビスティ".to_string(),
        1,
        319,
        18*5,
        60.1 + 1.2,
        7.5 - 0.4,
        11,
        770,
    );

    let eva15_6 = Model::new(
        "新世紀エヴァンゲリオン〜未来への咆哮〜".to_string(),
        "ビスティ".to_string(),
        4,
        319,
        8*2,
        70.0,
        99.4,
        163,
        1500,
    );

    exec_and_print(user_1, code_6, trials);
    exec_and_print(user_2, eva15_6, trials);
}

fn exec_and_print(mut user: User, mut model: Model, trials: usize) {
    // 遊戯開始、初当たりするまで回す
    let mut game_counts_until_first_win = Vec::new();
    for _ in 0..trials {
        let game_count = exec_until_first_win(model.jackpot_probability);
        game_counts_until_first_win.push(game_count);
    }
    game_counts_until_first_win.sort();

    // 初当たり、STモード結果をmodelに保存
    let total_balance = exec_st_and_cal_pay_out_ball(&mut model, &game_counts_until_first_win);
    user.balance = total_balance as isize;

    // 台スペック
    print_model_spec(&model);

    // 個人の結果
    println!("\n--------------{} 結果発表--------------", user.name);
    let sum = &game_counts_until_first_win.iter().sum::<usize>();
    let average = sum / &game_counts_until_first_win.len();
    println!("min回転 = {}", &game_counts_until_first_win[0]);
    println!("max回転 = {}", &game_counts_until_first_win[&game_counts_until_first_win.len()-1]);
    println!("average回転 = {}", average);
    let sign = if user.balance > 0 {"+"} else {""};
    println!("最終収支 {}{}円", sign, user.balance.separate_with_commas());
    let random_r = &game_counts_until_first_win[rand::thread_rng().gen_range(0, trials)];
    print_with_color((random_r/model.jackpot_probability)+1, format!("Todays Game First Jackpot count = {}\n", random_r));

    // 台の結果
    println!("\n--------------当たり履歴--------------");
    print_machine_results(&model);

    println!("\n");
}

fn exec_until_first_win(jackpot_probability: usize) -> usize {
    let mut game_count: usize = 0;

    loop {
        game_count += 1;

        if rand::thread_rng().gen_range(1, jackpot_probability+1) == rand::thread_rng().gen_range(1, jackpot_probability+1) {
            break;
        }

        if game_count >= MAX_TRAIAL {
            return MAX_TRAIAL+1;
        }
    }

    game_count
}

fn exec_st_and_cal_pay_out_ball(mut model: &mut Model, game_counts_until_first_win: &Vec<usize>) -> f64 {
    let mut total_balance: f64 = 0.0;

    // 大当たり & 確変集計
    for game_count in game_counts_until_first_win {
        let mut balance = 0.0;
        // let mut st_game_count = 0;
        let mut pay_out_ball = 0.0;

        // 大当たりに対する出玉(TODO 本当は確率によって違う)
        pay_out_ball += 450.0;

        let mut st_game_counts = vec![];
        // 確変に突入するかどうか
        if (rand::thread_rng().gen_range(0, 101) as f64) < model.st_rush_percentage {
            // 確変
            st_game_counts.push(0);
            let mut st_least = model.st_trials.clone();

            loop {
                if st_least <= 0 { break; }
                if rand::thread_rng().gen_range(1, model.st_probability as usize) == rand::thread_rng().gen_range(1, model.st_probability as usize) {
                    let st_trials = model.st_trials.clone();
                    st_game_counts.push(st_trials - st_least);
                    st_least = model.st_trials.clone();
                    pay_out_ball += model.st_bonus as f64;
                }
                st_least -= 1;
            }
        }

        // 出玉に金額を掛ける
        balance += pay_out_ball * model.price as f64;

        // MAX_TRAIALを超えていたら強制的に勝ち分を0にしている
        if game_count > &MAX_TRAIAL {
            balance = 0.0;
            st_game_counts = vec![];
        }

        // 初当たりまでに使った金額
        balance -= model.roll_per_cost * (*game_count as f64);

        let percentage =  (1 as f64-(((model.jackpot_probability-1) as f64 / model.jackpot_probability as f64).powf(*game_count as f64)))*100 as f64;
        let color_int = (game_count/model.jackpot_probability)+1;

        let sign = if balance > 0.0 {"+"} else {""};
        print_with_color(color_int, format!("{}回転({:.2}%)({}{:.2}円)(確変{}回),", game_count, percentage, sign, balance, st_game_counts.len()));

        total_balance += balance;

        let mut jackpot_count: HashMap<String, Vec<usize>> = HashMap::new();
        jackpot_count.insert(String::from("game_count"), vec![game_count.clone()]);
        jackpot_count.insert(String::from("st_game_counts"), st_game_counts);
        jackpot_count.insert(String::from("pay_out_ball"), vec![pay_out_ball as usize]);
        match model.jackpot_counts {
            Some(ref mut x) => x.push(jackpot_count),
            None    => model.jackpot_counts = Some(vec![jackpot_count]),
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
    // TODO iikanzinisitene
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

// TODO
// ありえない条件として、自分1人、同じ台を打ち続ける、金額は無限
// マルチスレッドで複数人が同時に打った場合でどうなるのかを検証しないと意味がない、その時行動モデルを選択できること、小遣いが切れたら帰ること
// 台としての収益、回転数、出玉、とかも計測できて、本当に１日のホールが終わった感じを作りたい、本当に万発出す台があるのか
// 養分、ハイエナ、回転重視、ランダム、どれがいいのだろうか
// 回転数は正規分布するはず、回転できる人間の判断材料をより明確にしたい
