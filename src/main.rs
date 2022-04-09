#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short, long)]
    number: Option<usize>,
}

#[paw::main]
fn main(args: Args) {
    let opt = Default::default();
    let mut words = voran::generate_words(&opt);

    if let Some(n) = args.number {
        words = words.take(n);
    }

    words.for_each(|w| println!("{w}"))
}
