use std::error::Error;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="sequ", about="Write a sequence to stdout.")]
struct Opt {
    #[structopt(short, long, help="min width of vals (0-padded)")]
    width: Option<usize>,
    lower: String,
    upper: Option<String>,
}

fn unwrap_exit<T>(result: Result<T, Box<dyn Error>>, msg: &str) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", msg, e);
            std::process::exit(-1);
        }
    }
}

fn parse_bound(b: String) -> Result<(usize, i64, bool), Box<dyn Error>> {
    let val: i64 = b.parse()?;
    let ds: Vec<char> = b.chars().collect();
    let nds = ds.len();
    let (digits, z) = if ds[0] == '-' {
        (nds - 1, nds > 1 && ds[1] == '0')
    } else {
        (nds, ds[0] == '0')
    };
    Ok((digits, val, z))
}

fn main() {
    let opt = Opt::from_args();
    let (lower, upper) = if let Some(s) = opt.upper {
        (opt.lower, s)
    } else {
        ("1".to_string(), opt.lower)
    };
    let (ldigits, lval, lz) =
        unwrap_exit(parse_bound(lower), "lower bound");
    let (udigits, uval, uz) =
        unwrap_exit(parse_bound(upper), "upper bound");
    let pdigits = if let Some(width) = opt.width {
        width
    } else if lz || uz {
        usize::max(ldigits, udigits)
    } else {
        0
    };
    for i in lval..=uval {
        println!("{1:00$}", pdigits, i);
    }
}
