extern crate rand;
use rand::Rng;

mod model;
use model::Model;
use thousands::Separable;

fn main() {
    let trials = 100;

    let eva15_6 = Model::new(
        "新世紀エヴァンゲリオン〜未来への咆哮〜".to_string(),
        "ビスティ".to_string(),
        4,
        319,
        4498,
        9*2,
    );

    let eva15_1 = Model::new(
        "新世紀エヴァンゲリオン〜未来への咆哮〜".to_string(),
        "ビスティ".to_string(),
        4,
        329,
        4498,
        8*2,
    );

    let aria = Model::new(
        "P緋弾のアリア～緋弾覚醒編～".to_string(),
        "藤商事".to_string(),
        1,
        199,
        2403,
        15*5,
    );

    let higrasi = Model::new(
        "Pひぐらしのなく頃に〜蕾〜".to_string(),
        "Daiichi".to_string(),
        4,
        129,
        1874,
        9*2,
    );

    exec_and_print(eva15_6, trials);
    exec_and_print(eva15_1, trials);
    exec_and_print(aria, trials);
    exec_and_print(higrasi, trials);
}

fn exec_and_print(model: Model, trials: usize) {
    // TODO collections
    let mut resurts = Vec::new();
    for _ in 0..trials {
        let count = exec(model.jackpot_probability);
        resurts.push(count);
    }

    println!("\n{}", model.name);
    println!("メーカー名 {}", model.maker);
    println!("大当り確率 1/{}(通常時)", model.jackpot_probability);
    println!("回転数 1000円あたり{}回転", model.roll_count_within_thousand);
    println!("1回転あたり{:.2}円", model.roll_per_cost);

    resurts.sort();
    print_result(model, trials, resurts);
}

fn exec(jackpot_probability: usize) -> usize {
    let mut count: usize = 0;

    loop {
        count += 1;

        if rand::thread_rng().gen_range(1, jackpot_probability+1) == rand::thread_rng().gen_range(1, jackpot_probability+1) {
            break;
        }
    }

    count
}

fn print_result(model: Model, trials: usize, resurts: Vec<usize>) {
    let mut total_balance: f64 = 0.0;

    for r in &resurts {
        let percentage =  (1 as f64-(((model.jackpot_probability-1) as f64 / model.jackpot_probability as f64).powf(*r as f64)))*100 as f64;
        let color_int = (r/model.jackpot_probability)+1;
        let balance: f64 = (model.jackpot_pay_out_average * model.price) as f64 - (model.roll_per_cost * (*r as f64));
        let sign = if balance > 0.0 {"+"} else {""};
        print_with_color(color_int, format!("{}回転({:.2}%)({}{:.2}円),", r, percentage, sign, balance));

        total_balance += balance;
    }
    println!("\n-------------------------");

    let sum = &resurts.iter().sum::<usize>();
    let average = sum / &resurts.len();

    println!("min = {}", &resurts[0]);
    println!("max = {}", &resurts[&resurts.len()-1]);
    println!("average = {}", average);
    let sign = if total_balance > 0.0 {"+"} else {""};
    println!("最終収支 {}{}円", sign, (total_balance as isize).separate_with_commas());

    let random_r = &resurts[rand::thread_rng().gen_range(0, trials)];
    print_with_color((random_r/model.jackpot_probability)+1, format!("Todays Game First Jackpot count = {}\n", random_r));
}

fn print_with_color(color_int: usize, text: String) {
    // TODO iikanzinisitene
    print!("\x1b[{}m{}\x1b[m ", 30+color_int, text);
}

// TODO
// ありえない条件として、自分1人、同じ台を打ち続ける、金額は無限、STモード度外視の計算
// マルチスレッドで複数人が同時に打った場合でどうなるのかを検証しないと意味がない
