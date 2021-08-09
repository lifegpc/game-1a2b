use crate::i18n::gettext;
use rand::Rng;
use std::io;

pub struct Game {
    pub len: i8,
    finshed: bool,
    num: u64,
}

fn check_num_is_vaild(num: u64, len: i8) -> bool {
    let mut vec: Vec<u8> = Vec::new();
    let mut temp = num;
    while temp > 0 {
        let n = (temp % 10) as u8;
        let mut iter = vec.iter();
        let r = iter.find(|&&x| x == n);
        match r {
            Some(_) => {
                return false;
            }
            None => {
                vec.push(n);
                temp = temp / 10;
            }
        }
    }
    if vec.len() != (len as usize) {
        return false;
    }
    return true;
}

struct CompareResult {
    a: i8,
    b: i8,
    ok: bool,
}

impl CompareResult {
    fn print(&self) {
        println!("{}A{}B", self.a, self.b);
    }
}

fn split_num(num: u64, vec: &mut Vec<u8>) {
    vec.clear();
    let mut temp = num;
    while temp > 0 {
        let n = (temp % 10) as u8;
        vec.push(n);
        temp = temp / 10;
    }
    vec.reverse();
}

fn compare_num(sec: u64, num: u64, len: i8) -> CompareResult {
    let mut sec_vec: Vec<u8> = Vec::new();
    split_num(sec, &mut sec_vec);
    let mut num_vec: Vec<u8> = Vec::new();
    split_num(num, &mut num_vec);
    let mut a: i8 = 0;
    let mut b: i8 = 0;
    for i in 0..len {
        if sec_vec[i as usize] == num_vec[i as usize] {
            a += 1;
        }
        let mut iter = sec_vec.iter();
        let r = iter.find(|&&x| x == num_vec[i as usize]);
        match r {
            Some(_) => {
                b += 1;
            }
            None => {}
        }
    }
    b -= a;
    let ok = a == len;
    return CompareResult { a: a, b: b, ok: ok };
}

fn get_num_from_stdin() -> u64 {
    loop {
        let mut input = String::new();
        let re = io::stdin().read_line(&mut input);
        match re {
            Ok(_) => {}
            Err(..) => {
                continue;
            }
        }
        let trimed = input.trim();
        if trimed.len() == 0 {
            continue;
        }
        let i = trimed.parse::<u64>();
        match i {
            Ok(_) => {}
            Err(..) => {
                println!("{} {}", gettext("This was not an integer:"), trimed);
                continue;
            }
        };
        return i.unwrap();
    }
}

impl Game {
    pub fn new(len: i8) -> Option<Game> {
        if len >= 1 && len <= 10 {
            Some(Game {
                len: len,
                finshed: false,
                num: u64::MAX,
            })
        } else {
            None
        }
    }

    pub fn start(&mut self) {
        if self.finshed {
            return;
        }
        let base: u64 = 10;
        let min: u64 = base.pow((self.len - 1) as u32);
        let max: u64 = base.pow(self.len as u32);
        let s = gettext("Start to genrate random number which have <num> digits.").replace("<num>", format!("{}", self.len).as_str());
        println!("{}", s);
        let mut rng = rand::thread_rng();
        self.num = rng.gen_range(min..max);
        while !check_num_is_vaild(self.num, self.len) {
            self.num = rng.gen_range(min..max);
        }
        println!("{}", gettext("Genrate successfully."));
        loop {
            let num = get_num_from_stdin();
            if !check_num_is_vaild(num, self.len) {
                let s = gettext("<input> is not a vaild input.").replace("<input>", format!("{}", num).as_str());
                println!("{}", s);
                continue;
            }
            let r = compare_num(self.num, num, self.len);
            if r.ok {
                println!("{}", gettext("You win!"));
                self.finshed = true;
                return;
            }
            r.print();
        }
    }
}
