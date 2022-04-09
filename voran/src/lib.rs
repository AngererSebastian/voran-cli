use rand::seq::IteratorRandom;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Options {
    pub max_len: usize,
    pub min_len: usize,
    // TODO: get this into the algorithm
    pub max_constant_chain: usize,
    pub vowels: Vec<String>,
    pub constants: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_len: 12,
            min_len: 6,
            max_constant_chain: 2,
            vowels: include_str!("vowels.txt")
                .lines()
                .map(|v| v.to_string())
                .collect(),
            constants: include_str!("constants.txt")
                .lines()
                .map(|c| c.to_string())
                .collect(),
        }
    }
}

pub fn generate_word_with_rng<R: rand::Rng>(opt: &Options, rng: &mut R) -> String {
    let mut chars_left = rng.gen_range(opt.min_len..=opt.max_len);

    let mut was_voc = false;
    //let mut kind_count = 0;

    let mut result = String::new();

    while chars_left > 0 {
        let next =
        // 70% chance there is a non voc after a vocal
        if was_voc && rng.gen_bool(0.7) {
            was_voc = false;
            random_elem_with_len(&opt.constants, chars_left, rng)
        }
        else {
            was_voc = true;
            random_elem_with_len(&opt.vowels, chars_left, rng)
        };

        chars_left -= next.len();
        result.push_str(next);
    }

    result
}

fn random_elem_with_len<'a, R: rand::Rng>(
    xs: &'a [String],
    max_len: usize,
    rng: &mut R,
) -> &'a str {
    xs.iter()
        .filter(|x| x.len() <= max_len)
        .choose(rng)
        .expect("no chars?")
}

pub fn generate_word(opt: &Options) -> String {
    let mut rng = rand::thread_rng();
    generate_word_with_rng(opt, &mut rng)
}

pub fn generate_words<'a>(opt: &'a Options) -> impl Iterator<Item = String> + 'a {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(move || generate_word_with_rng(opt, &mut rng))
}
