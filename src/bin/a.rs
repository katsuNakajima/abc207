#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let abc = parse_vec!(u32);
    let min = abc.clone().into_iter().min().unwrap();
    let sum: u32 = abc.into_iter().sum();
    let ans = sum - min;
    println!("{}", ans);
}
