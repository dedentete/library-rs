use rand::Rng;
use rand_pcg::*;

fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}
struct Input {
}

impl Input {
    fn new() -> Input {

    }
}

#[derive(Clone)]
struct State {
    score, 
}

impl State {
    fn new(input: &Input) -> State {
        
    }
}

fn solve(input: &Input) -> Vec<i32> {
    const TL: f64 = 1.9;
    let mut rng = rand_pcg::Pcg64Mcg::new(890482);
    let mut step = 0;
    let mut state = State::new(input);
    let mut start_temp;
    let mut end_temp;
    loop {
        let t = get_time() / TL;
        if t > 1.0 {
            break;
        }
        let temp = start_temp + (end_temp - start_temp) * t;
        if newstate.score > state.score {
            state = newstate;
        }
        step += 1;
    }
    eprintln!("score : {}", state.score);
    eprintln!("step : {}", step);
    state.t
}
