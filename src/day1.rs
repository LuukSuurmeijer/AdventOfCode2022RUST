use std::io::{Read};
use std::fs::File;

pub fn read_data(s: &str) -> Vec<Vec<usize>> {
    let mut file = File::open(s).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut data: Vec<Vec<usize>> = Vec::new();
    let mut elf: Vec<usize> = Vec::new();
    for line in s.lines() {
        if line == "" {
            data.push(elf.to_owned()); // without to_owned() we get a "value moved in previous iteration" error
            elf.clear();
        }
        else {
            elf.push(line.parse::<usize>().unwrap());
        }
    }

    // The data is one large string consisting of 1\n2\n\n3\n4\n\5n\n etc.
    // So after every double \n, the rations of a new Elf start
    // 1. We first split the data on "\n\n" to get an iterator over each Elf
    // 2. For each Elf, we need to parse their rations (a string eg "1\n2") into a vector of numbers
    // I use a functional-style approach here with iterators and map(), at the end I collect everything into a vector of vectors
    //
    //let data: Vec<Vec<usize>> = s.split("\n\n").into_iter()
    //                                    .map(|elf| elf.split('\n').into_iter()
    //                                        .map(|ration| ration.parse::<usize>().unwrap()).collect())
    //                                .collect();
    //
    // This does not work because split does not work on "\n\n" for some unknown reason :-))))

    data

}

pub fn get_max(calories: &Vec<Vec<usize>>) -> usize {

    // Iterate over the Elf's with map and sum their values, then take the max over the resulting iterator
    calories.into_iter()
    .map(|elf| elf.into_iter().sum::<usize>())
        .max()
    .unwrap()
}

pub fn get_max_n(calories: &Vec<Vec<usize>>, n: usize) -> usize {

    // Same principle as above, instead here we sort the resuling vector and sum the last N items

    let mut sums: Vec<usize> = calories.into_iter().map(|elf| elf.into_iter().sum::<usize>()).collect();
    
    sums.sort();

    sums[sums.len()-n..sums.len()].into_iter().sum::<usize>()
}