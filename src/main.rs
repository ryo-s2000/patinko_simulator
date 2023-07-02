extern crate rand;
use rand::Rng;

mod model;
use model::Model;
use thousands::Separable;

static MAX_TRAIAL: usize = 600;

fn main() {
    let trials = 12;

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

    exec_and_print(eva15_6, trials);
}

fn exec_and_print(model: Model, trials: usize) {
    let mut game_counts_until_first_win = Vec::new();
    for _ in 0..trials {
        let game_count = exec_until_first_win(model.jackpot_probability);
        game_counts_until_first_win.push(game_count);
    }

    println!("\n{}", model.name);
    println!("メーカー名 {}", model.maker);
    println!("大当り確率 1/{}(通常時) 1/{}(高確率時)", model.jackpot_probability, model.st_probability);
    println!("回転数 1000円あたり{}回転", model.roll_count_within_thousand);
    println!("1回転あたり{:.2}円", model.roll_per_cost);

    game_counts_until_first_win.sort();
    print_result(model, trials, game_counts_until_first_win);
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

fn print_result(model: Model, trials: usize, game_counts_until_first_win: Vec<usize>) {
    let mut total_balance: f64 = 0.0;

    for game_count in &game_counts_until_first_win {
        let mut balance = 0.0;
        let mut st_count = 0;
        let mut pay_out_ball = 0.0;

        // 大当たりに対する出玉(TODO 本当は確率によって違う)
        pay_out_ball += 450.0;

        // 確変に突入するかどうか
        if (rand::thread_rng().gen_range(0, 101) as f64) < model.st_rush_percentage {
            // 確変
            st_count += 1;
            let mut st_least = model.st_trials.clone();

            loop {
                if st_least <= 0 { break; }
                if rand::thread_rng().gen_range(1, model.st_probability as usize) == rand::thread_rng().gen_range(1, model.st_probability as usize) {
                    st_count += 1;
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
            st_count = 0;
        }

        // 初当たりまでに使った金額
        balance -= model.roll_per_cost * (*game_count as f64);

        let percentage =  (1 as f64-(((model.jackpot_probability-1) as f64 / model.jackpot_probability as f64).powf(*game_count as f64)))*100 as f64;
        let color_int = (game_count/model.jackpot_probability)+1;

        let sign = if balance > 0.0 {"+"} else {""};
        print_with_color(color_int, format!("{}回転({:.2}%)({}{:.2}円)(確変{}回),", game_count, percentage, sign, balance, st_count));

        total_balance += balance;
    }

    println!("\n-------------------------");

    let sum = &game_counts_until_first_win.iter().sum::<usize>();
    let average = sum / &game_counts_until_first_win.len();

    println!("min = {}", &game_counts_until_first_win[0]);
    println!("max = {}", &game_counts_until_first_win[&game_counts_until_first_win.len()-1]);
    println!("average = {}", average);
    let sign = if total_balance > 0.0 {"+"} else {""};
    println!("最終収支 {}{}円", sign, (total_balance as isize).separate_with_commas());

    let random_r = &game_counts_until_first_win[rand::thread_rng().gen_range(0, trials)];
    print_with_color((random_r/model.jackpot_probability)+1, format!("Todays Game First Jackpot count = {}\n", random_r));
}

fn print_with_color(color_int: usize, text: String) {
    // TODO iikanzinisitene
    print!("\x1b[{}m{}\x1b[m ", 30+color_int, text);
}

// TODO
// ありえない条件として、自分1人、同じ台を打ち続ける、金額は無限、STモード度外視の計算
// マルチスレッドで複数人が同時に打った場合でどうなるのかを検証しないと意味がない
