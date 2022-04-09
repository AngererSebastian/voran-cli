#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short, long)]
    number: Option<usize>,
}

#[paw::main]
fn main(args: Args) {
    let opt = Default::default();
    let mut words: Box<dyn Iterator<Item = String>> = Box::new(voran::generate_words(&opt));

    if let Some(n) = args.number {
        words = Box::new(words.take(n));
    }

    words.for_each(|w| println!("{w}"))
}
