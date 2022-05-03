use rand::seq::IteratorRandom;

#[cfg(test)]
mod tests {
    #[test]
    fn length_stays_in_bounds() {
        let opts = super::Options {
            max_len: 12,
            min_len: 6,
            ..Default::default()
        };
        let words = super::generate_words(&opts);

        words
            .take(50)
            .map(|w| w.len() >= opts.min_len && w.len() <= opts.max_len)
            .for_each(|b| assert!(b))
    }
}

/// options for random word generation
pub struct Options {
    /// the max length of any word in characters
    pub max_len: usize,
    /// the minimum length of any word in characters
    pub min_len: usize,
    // TODO: get this into the algorithm
    pub max_constant_chain: usize,
    /// the vowels used for generation
    /// ## default
    /// the vowels inside `src/vowels.txt`
    pub vowels: Vec<String>,
    /// the constants used for generation
    /// ## default
    /// the vowels inside `src/constants.txt`
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

/// generate one word with the given rng
pub fn generate_word_with_rng<R: rand::Rng>(opt: &Options, rng: &mut R) -> String {
    let size = rng.gen_range(opt.min_len..=opt.max_len);

    let mut was_voc = false;
    let mut vowels_count = 0;

    let mut result = if rng.gen_bool(0.5) {
        was_voc = true;
        vowels_count = 1;
        random_elem_with_len(&opt.vowels, size, rng)
    } else {
        random_elem_with_len(&opt.constants, size, rng)
    }
    .to_string();

    while size - result.len() > 0 {
        let left = size - result.len();
        let next =
        // 70% chance there is a non voc after a vocal
        if was_voc && rng.gen_bool(0.7 + vowels_count as f64 * 0.1) {
            vowels_count = 0;
            was_voc = false;
            random_elem_with_len(&opt.constants, left, rng)
        }
        else {
            if !was_voc { vowels_count += 1;}
            was_voc = true;
            random_elem_with_len(&opt.vowels, left, rng)
        };

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

/// generate a random word, uses thread_rng
pub fn generate_word(opt: &Options) -> String {
    let mut rng = rand::thread_rng();
    generate_word_with_rng(opt, &mut rng)
}

/// generate a never ending iterator of random words, uses thread_rng
pub fn generate_words<'a>(opt: &'a Options) -> impl Iterator<Item = String> + 'a {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(move || generate_word_with_rng(opt, &mut rng))
}

/// generate never ending random words, with given rng
pub fn generate_words_with_rng<'a, R: rand::Rng>(opt: &'a Options, rng: &'a mut R) -> impl Iterator<Item = String> + 'a{
    std::iter::repeat_with(move || generate_word_with_rng(opt, rng))
}