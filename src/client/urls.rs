pub(crate) const DEFAULT_BASE_URL: &str = "https://api.geocod.io/v1.9/";
pub(crate) const DEFAULT_GEOCODE_URL: &str = "https://api.geocod.io/v1.9/geocode";

pub(crate) fn build_url(base_url: &str, route: &str) -> String {
    let mut out = String::from(base_url);

    if !out.ends_with('/') {
        out.push('/');
    }
    out.push_str(route);

    out
}
