use rand::Rng;

fn main() {
    // TODO enum and struct
    // TODO ST mode
    let hatuatari = 319;
    let sikou = 300;

    // TODO collections
    // TODO multithread
    // TODO 収支計算
    let mut resurts = Vec::new();
    for _ in 0..sikou {
        let count = exec(hatuatari);
        resurts.push(count);
    }

    resurts.sort();
    print_result(resurts, hatuatari, sikou);
}

fn exec(hatuatari: usize) -> usize {
    let mut count: usize = 0;

    loop {
        count += 1;

        if rand::thread_rng().gen_range(1, hatuatari+1) == rand::thread_rng().gen_range(1, hatuatari+1) {
            break;
        }
    }

    count
}

fn print_result(resurts: Vec<usize>, hatuatari: usize, sikou: usize) {
    for r in &resurts {
        let percentage =  (1 as f64-(((hatuatari-1) as f64 / hatuatari as f64).powf(*r as f64)))*100 as f64;
        print_with_color((r/hatuatari)+1, format!("{}({:.2}%),", r, percentage));
    }
    println!("\n-------------------------");

    let sum = &resurts.iter().sum::<usize>();
    let average = sum / &resurts.len();

    println!("min = {}", &resurts[0]);
    println!("max = {}", &resurts[&resurts.len()-1]);
    println!("average = {}", average);

    let random_r = &resurts[rand::thread_rng().gen_range(0, sikou)];
    print_with_color((random_r/hatuatari)+1, format!("Todays Game hatuatari = {}\n", random_r));
}

fn print_with_color(color_int: usize, text: String) {
    // TODO iikanzinisitene
    print!("\x1b[{}m{}\x1b[m ", 30+color_int, text);
}
