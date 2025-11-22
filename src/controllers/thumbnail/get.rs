use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};

const EMOJI_MAP: &[(&str, &str)] = &[
    ("cry", "\u{1F622}"),
    ("laugh", "\u{1F602}"),
    ("smile", "\u{1F60A}"),
    ("heart", "\u{2764}\u{FE0F}"),
    ("fire", "\u{1F525}"),
    ("star", "\u{2B50}"),
    ("rocket", "\u{1F680}"),
    ("check", "\u{2705}"),
    ("cross", "\u{274C}"),
    ("warning", "\u{26A0}\u{FE0F}"),
    ("info", "\u{2139}\u{FE0F}"),
    ("question", "\u{2753}"),
    ("exclamation", "\u{2757}"),
    ("wrench", "\u{1F527}"),
    ("debug", "\u{1F41B}"),
    ("developer", "\u{1F468}\u{200D}\u{1F4BB}"),
    ("building", "\u{1F3D7}\u{FE0F}"),
];

#[derive(Debug)]
enum BackgroundColor {
    Solid(String),
    Gradient(String, String),
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor::Gradient("#667eea".to_string(), "#764ba2".to_string())
    }
}

#[derive(Debug)]
struct ThumbnailConfig {
    emoji: String,
    title: String,
    body: String,
    font_size: u32,
    width: u32,
    height: u32,
    emoji_size: u32,
    background_color: BackgroundColor,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            emoji: String::new(),
            title: String::new(),
            body: String::new(),
            font_size: 48,
            width: 1200,
            height: 630,
            emoji_size: 120,
            background_color: BackgroundColor::default(),
        }
    }
}

fn get_emoji(key: &str) -> String {
    EMOJI_MAP
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| key.to_string())
}

fn is_hex(color: &str) -> bool {
    color.len() == 6 && color.chars().all(|c| c.is_ascii_hexdigit())
}

fn parse_color(color: &str) -> String {
    let trimmed = color.trim();
    if is_hex(trimmed) {
        format!("#{}", trimmed)
    } else {
        trimmed.to_string()
    }
}

fn parse_thumbnail_path(path: &str) -> Option<ThumbnailConfig> {
    let path = path.trim_end_matches(".svg");
    let parts: Vec<&str> = path.split('-').collect();

    if parts.len() < 2 || parts[0] != "image" {
        return None;
    }

    let mut config = ThumbnailConfig::default();

    for arg in parts.iter().skip(1) {
        if let Some((key, value)) = arg.split_once(':') {
            match key {
                "emoji" => config.emoji = get_emoji(value),
                "title" => config.title = value.to_string(),
                "body" => config.body = value.to_string(),
                "fontSize" => {
                    if let Ok(size) = value.parse() {
                        config.font_size = size;
                    }
                }
                "backgroundColor" => {
                    if value.contains(',') {
                        let colors: Vec<&str> = value.split(',').collect();
                        if colors.len() >= 2 {
                            config.background_color = BackgroundColor::Gradient(
                                parse_color(colors[0]),
                                parse_color(colors[1]),
                            );
                        }
                    } else {
                        config.background_color = BackgroundColor::Solid(parse_color(value));
                    }
                }
                _ => {}
            }
        }
    }

    Some(config)
}

fn calculate_vertical_spacing(texts: &[&str], spacing: &[f64]) -> Vec<f64> {
    let count = texts.len();
    if count == 1 {
        return vec![0.0];
    }

    let mut positions = Vec::with_capacity(count);
    for i in 0..count {
        let offset = (i as f64) - ((count - 1) as f64) / 2.0;
        let space = spacing.get(i).copied().unwrap_or(0.0);
        positions.push(offset * space);
    }

    positions
}

fn generate_svg(config: &ThumbnailConfig) -> String {
    let ThumbnailConfig {
        emoji,
        title,
        body,
        font_size,
        width,
        height,
        emoji_size,
        background_color,
    } = config;

    let has_title = !title.is_empty();
    let has_body = !body.is_empty();

    let mut texts: Vec<&str> = Vec::new();
    let mut spacing: Vec<f64> = Vec::new();

    if !emoji.is_empty() {
        texts.push(emoji.as_str());
        spacing.push(*emoji_size as f64);
    }
    if has_title {
        texts.push(title.as_str());
        spacing.push(*font_size as f64);
    }
    if has_body {
        texts.push(body.as_str());
        spacing.push(*font_size as f64 * 0.75);
    }

    let positions = calculate_vertical_spacing(&texts, &spacing);

    let mut emoji_y = 0.0;
    let mut title_y = 0.0;
    let mut body_y = 0.0;

    let mut pos_idx = 0;
    if !emoji.is_empty() {
        emoji_y = positions.get(pos_idx).copied().unwrap_or(0.0);
        pos_idx += 1;
    }
    if has_title {
        title_y = positions.get(pos_idx).copied().unwrap_or(0.0);
        pos_idx += 1;
    }
    if has_body {
        body_y = positions.get(pos_idx).copied().unwrap_or(0.0);
    }

    let (gradient_def, fill_value) = match background_color {
        BackgroundColor::Gradient(start, end) => (
            format!(
                r#"<linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" style="stop-color:{};stop-opacity:1" />
            <stop offset="100%" style="stop-color:{};stop-opacity:1" />
          </linearGradient>"#,
                start, end
            ),
            "url(#bg)".to_string(),
        ),
        BackgroundColor::Solid(color) => (String::new(), color.clone()),
    };

    let emoji_element = if !emoji.is_empty() {
        format!(
            r#"<text x="0" y="{}" text-anchor="middle" dominant-baseline="middle" font-size="{}" font-family="Arial, sans-serif">{}</text>"#,
            emoji_y, emoji_size, emoji
        )
    } else {
        String::new()
    };

    let title_element = if has_title {
        format!(
            r#"<text x="0" y="{}" text-anchor="middle" dominant-baseline="middle" font-size="{}" font-family="Arial, sans-serif" font-weight="bold" fill="white">{}</text>"#,
            title_y,
            font_size,
            html_escape(title)
        )
    } else {
        String::new()
    };

    let body_element = if has_body {
        format!(
            r#"<text x="0" y="{}" text-anchor="middle" dominant-baseline="middle" font-size="{}" font-family="Arial, sans-serif" fill="rgba(255,255,255,0.8)">{}</text>"#,
            body_y,
            (*font_size as f64 * 0.8) as u32,
            html_escape(body)
        )
    } else {
        String::new()
    };

    format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
    <defs>
      {}
    </defs>
    <rect width="{}" height="{}" fill="{}" />
    <g transform="translate({}, {})">
      {}
      {}
      {}
    </g>
  </svg>"#,
        width,
        height,
        gradient_def,
        width,
        height,
        fill_value,
        width / 2,
        height / 2,
        emoji_element,
        title_element,
        body_element
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub async fn get(Path(path): Path<String>) -> impl IntoResponse {
    let config = match parse_thumbnail_path(&path) {
        Some(c) => c,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid path format"))
                .unwrap()
        }
    };

    let svg_content = generate_svg(&config);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(Body::from(svg_content))
        .unwrap()
}
