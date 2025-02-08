pub fn generate_slug(title: &str) -> String {
    // Convert title to lowercase
    let title = title.to_lowercase();

    // Replace spaces with hyphens
    let mut slug = title.replace(' ', "-");

    // Remove characters that are not alphanumeric or hyphens
    slug = slug.chars().filter(|c| c.is_alphanumeric() || *c == '-').collect();

    // Trim leading or trailing hyphens
    slug = slug.trim_matches('-').to_string();

    slug
}