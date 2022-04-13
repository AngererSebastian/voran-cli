#[derive(structopt::StructOpt)]
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
}

#[paw::main]
fn main(args: Args) {
    let opt = voran::Options {
        min_len: args.min,
        max_len: args.max,
        ..Default::default()
    };
    let mut words: Box<dyn Iterator<Item = String>> = Box::new(voran::generate_words(&opt));

    if let Some(n) = args.number {
        words = Box::new(words.take(n));
    }

    words.for_each(|w| println!("{w}"))
}
