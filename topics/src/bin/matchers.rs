use refs::matchers;

pub fn main() {
    matchers::match_partners(matchers::CheckRange::Hourly(matchers::CustomRange::Random));
    matchers::options_match();
}
