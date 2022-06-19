#![feature(box_syntax)]
#[derive(structopt::StructOpt)]
#[structopt(about = "generate random voicable names")]
struct Args {
    /// how many names should be generated,
    /// when not specified, creates a continous stream of names
    #[structopt(short, long)]
    number: Option<usize>,
    /// the minimum length of the name
    #[structopt(long, short = "m", default_value = "6")]
    min: usize,
    // the maximum length of the name
    #[structopt(long, short = "x", default_value = "12")]
    max: usize,
    #[structopt(short, long)]
    regex: Option<String>,
}

#[paw::main]
fn main(args: Args) {
    let opt = voran::Options {
        min_len: args.min,
        max_len: args.max,
        ..Default::default()
    };
    let mut words: Box<dyn Iterator<Item = String>> = box voran::generate_words(&opt);

    if let Some(r) = args.regex {
        let regex = regex::Regex::new(&r).expect("invalid regex");
        words = box words.filter(move |w| regex.is_match(w));
    }

    if let Some(n) = args.number {
        words = box words.take(n);
    }

    words.for_each(|w| println!("{w}"))
}
