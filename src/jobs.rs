pub struct Job<'a> {
    name: &'a str,
    affinities: Affinities<'a>,
}

pub struct Affinities<'a> {
    _50: &'a str,
    _120: &'a str,
    _250: &'a str,
    _400: &'a str,
    _600: &'a str,
}
