use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::process::exit;

fn main() {
    let fastin = fs::read_to_string("./fastin.txt").unwrap();
    let lines = fastin.lines().collect::<Vec<&str>>();

    let size = match lines[0].parse::<i128>() {
        Ok(res) => res,
        Err(err) => {
            println!("err:1 {}", err);
            exit(0);
        }
    };

    let bu = match lines[1].parse::<i128>() {
        Ok(res) => res,
        Err(err) => {
            println!("err:2 {}", err);
            exit(0);
        }
    };

    if size % 8 != 0 {
        println!("size shall be divisable by  8");
        exit(0);
    }

    let mut sum = 0;
    let mut public = vec![];
    for i in 2..size + 2 {
        let buf = lines[i as usize];
        let val = match buf.parse::<i128>() {
            Ok(res) => {
                sum += res;
                res
            }
            Err(err) => {
                println!("err3: {}", err);
                exit(0);
            }
        };
        public.append(&mut vec![val]);
    }

    let max = match public.iter().max() {
        Some(max) => *max,
        None => exit(0),
    };
    let min = match public.iter().min() {
        Some(min) => *min,
        None => exit(0),
    };
    println!(
        "size:{}, U:{}, max:{}, min:{}, sum:{}\n{:?}",
        size,
        bu,
        max,
        min,
        sum,
        public.clone()
    );

    let pairs = (1..bu + 1)
        .map(|i| {
            let mut vec = vec![];
            for j in i + 1..bu + 1 {
                if euc_lib::I128::euc(i, j) == 1 {
                    vec.append(&mut vec![(i, j)]);
                }
            }
            return vec;
        })
        .collect::<Vec<Vec<(i128, i128)>>>();

    let pairs = pairs
        .into_iter()
        .flatten()
        .unique()
        .collect::<Vec<(i128, i128)>>();
    println!("workable pairs created: {}", pairs.len());

    let options = pairs
        .par_iter()
        .map(|t| {
            let (i, j) = t;
            for n in max..(max+(sum/size)) {
                for q in bu..n {
                    if euc_lib::I128::euc(q, n) == 1
                        && public[0] == (i * q) % n
                        && public[1] == (j * q) % n
                    {
                        return (*i, *j, q, n);
                    }
                }
            }
            return (0, 0, 0, 0);
        })
        .collect::<Vec<(i128, i128, i128, i128)>>();

    let options = options
        .iter()
        .unique()
        .collect::<Vec<&(i128, i128, i128, i128)>>();
    println!("working options created, {}, {:?}", options.len(), options);

    let estim = options
        .par_iter()
        .map(|t| {
            let (_, _, q, n) = t;
            let mut count = 0;
            let mut prev = 1;
            for i in 0..public.len() {
                for j in prev..*n {
                    if public[i] == (j * q) % n {
                        count += 1;
                        prev = j;
                        break;
                    }
                }
            }
            if count == 8 {
                return (*q, *n);
            }
            return (0, 0);
        })
        .collect::<Vec<(i128, i128)>>();

    let mut estim = estim
        .iter()
        .unique()
        .sorted()
        .collect::<Vec<&(i128, i128)>>();

    estim.sort();

    println!("first estimation, {}, {:?}", estim.len(), estim);
}