use crate::models::Repo;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Bump when client cache shape or sort semantics change (invalidates old localStorage).
const CACHE_KEY: &str = "portfolio_repos_v2";
/// Max GitHub API pages (100 repos each) to avoid unbounded requests.
const MAX_REPO_PAGES: u32 = 10;

/// Grid order: C++ showcases first (Rubik, 4D-cube), then board games and the rest.
/// Repos not listed here sort after: **C++ (and C) before other languages**, then by name.
const REPO_DISPLAY_ORDER: &[&str] = &[
    "Rubik",
    "4D-cube",
    "Go",
    "Latrones",
    "Game-of-Ur",
    "Chaturanga",
    "Nard",
    "Senet",
    "Mehen",
    "Bria-ai",
    "Aerospace",
    "Silent-data-corruption",
    "Liquid",
];

/// Priority for unknown repos: C++ and C before others (future-proof “feature C++ first”).
fn language_sort_key(lang: Option<&str>) -> u8 {
    match lang {
        Some("C++") => 0,
        Some("C") => 1,
        _ => 2,
    }
}

/// Stable sort: `REPO_DISPLAY_ORDER` first; unlisted repos use C/C++ before other languages, then name.
fn sort_repos_for_display(repos: &mut Vec<Repo>) {
    let unknown = REPO_DISPLAY_ORDER.len();
    repos.sort_by(|a, b| {
        let ia = REPO_DISPLAY_ORDER
            .iter()
            .position(|&n| n == a.name.as_str())
            .unwrap_or(unknown);
        let ib = REPO_DISPLAY_ORDER
            .iter()
            .position(|&n| n == b.name.as_str())
            .unwrap_or(unknown);
        ia.cmp(&ib).then_with(|| {
            if ia == unknown && ib == unknown {
                language_sort_key(a.language.as_deref())
                    .cmp(&language_sort_key(b.language.as_deref()))
                    .then_with(|| a.name.cmp(&b.name))
            } else {
                a.name.cmp(&b.name)
            }
        })
    });
}

/// Repo names omitted from the public grid (e.g. this site’s own repo).
const HIDDEN_FROM_GRID: &[&str] = &["azuree0-portfolio.pages.dev"];

fn filter_hidden_from_grid(repos: &mut Vec<Repo>) {
    repos.retain(|r| !HIDDEN_FROM_GRID.contains(&r.name.as_str()));
}

/// Sort, then drop repos not shown on the portfolio grid.
fn finalize_repos_for_display(mut repos: Vec<Repo>) -> Vec<Repo> {
    sort_repos_for_display(&mut repos);
    filter_hidden_from_grid(&mut repos);
    repos
}

#[derive(Serialize, Deserialize)]
struct CachedRepos {
    repos: Vec<Repo>,
    fetched_at: u64,
}

/// Instant paint: last successful fetch from localStorage, else hardcoded fallback.
/// Sorts, applies grid filters (e.g. hides this repo’s own GitHub project card).
pub fn initial_repos() -> Vec<Repo> {
    if let Ok(cached) = get_cached() {
        return finalize_repos_for_display(cached.repos);
    }
    finalize_repos_for_display(static_fallback())
}

/// Static fallback repos when API fails (azuree0's known repos)
pub fn static_fallback() -> Vec<Repo> {
    let base = "https://github.com/azuree0";
    vec![
        Repo {
            name: "Go".to_string(),
            description: Some("Go board game".to_string()),
            html_url: format!("{}/Go", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/f52cbdc4-afe9-4169-8a03-ed025b6a834a".to_string()),
        },
        Repo {
            name: "Latrones".to_string(),
            description: Some("Roman board game".to_string()),
            html_url: format!("{}/Latrones", base),
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/182fd35b-a924-4749-8f37-9f48060ec49f".to_string()),
        },
        Repo {
            name: "Game-of-Ur".to_string(),
            description: Some("Royal Game of Ur".to_string()),
            html_url: format!("{}/Game-of-Ur", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/fe00622d-3483-47b2-b9ff-b00a17f4c159".to_string()),
        },
        Repo {
            name: "Chaturanga".to_string(),
            description: Some("Ancient Indian chess precursor".to_string()),
            html_url: format!("{}/Chaturanga", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/ebd040e8-8939-4ad7-bc43-e655a4ba6582".to_string()),
        },
        Repo {
            name: "Nard".to_string(),
            description: Some("Backgammon variant".to_string()),
            html_url: format!("{}/Nard", base),
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/9cdc289f-cd3d-433f-af37-d508c45c7179".to_string()),
        },
        Repo {
            name: "Senet".to_string(),
            description: Some("One of the oldest known board games, dating back to ancient Egypt (around 3100 BCE). Played on 30 squares in three rows. Players move pieces based on dice throws, with special rules for squares like the House of Water and House of Happiness. Senet means 'passing' in ancient Egyptian—the soul's passage through the underworld.".to_string()),
            html_url: format!("{}/Senet", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/b12746e5-fb64-41a4-b343-5ec77166cff6".to_string()),
        },
        Repo {
            name: "Mehen".to_string(),
            description: Some("Ancient Egyptian snake game".to_string()),
            html_url: format!("{}/Mehen", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/b9a324c1-822d-49ed-b88e-13fbc2b17f04".to_string()),
        },
        Repo {
            name: "Rubik".to_string(),
            description: Some("C++ project".to_string()),
            html_url: format!("{}/Rubik", base),
            language: Some("C++".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/5de4e3d7-b660-4ea4-a513-aca077b695cf".to_string()),
        },
        Repo {
            name: "4D-cube".to_string(),
            description: Some("4D tesseract: SFML/OpenGL, 4D rotations and projection.".to_string()),
            html_url: format!("{}/4D-cube", base),
            language: Some("C++".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some(
                "https://github.com/user-attachments/assets/267af065-af29-4cad-8075-786b07822982"
                    .to_string(),
            ),
        },
        Repo {
            name: "Bria-ai".to_string(),
            description: Some("Python AI project".to_string()),
            html_url: format!("{}/Bria-ai", base),
            language: Some("Python".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/ed5d7f08-27a7-40fb-a93a-6f2b4b89cab4".to_string()),
        },
        Repo {
            name: "Aerospace".to_string(),
            description: Some("Python project".to_string()),
            html_url: format!("{}/Aerospace", base),
            language: Some("Python".to_string()),
            stargazers_count: 2,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/86f9cac2-334b-4cf7-b374-d82dba4bd186".to_string()),
        },
        Repo {
            name: "Silent-data-corruption".to_string(),
            description: Some("C++ project".to_string()),
            html_url: format!("{}/Silent-data-corruption", base),
            language: Some("C++".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/1db52073-faf9-4e6d-895c-36c66dc1625d".to_string()),
        },
        Repo {
            name: "Liquid".to_string(),
            description: Some("Liquid templates".to_string()),
            html_url: format!("{}/Liquid", base),
            language: Some("Liquid".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/ac0f0af2-e95e-4591-b848-e30c89675822".to_string()),
        },
    ]
}

/// Fetches all public repos from GitHub (paginated), always hitting the network when possible.
/// On failure, returns last cached list if any. Merges screenshots from static fallback.
pub async fn fetch_repos() -> Result<Vec<Repo>, String> {
    let mut repos: Vec<Repo> = Vec::new();

    for page in 1..=MAX_REPO_PAGES {
        let url = format!(
            "https://api.github.com/users/azuree0/repos?sort=updated&per_page=100&page={}",
            page
        );
        let response = match gloo_net::http::Request::get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "azuree0-portfolio-pages")
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                if let Ok(cached) = get_cached() {
                    return Ok(finalize_repos_for_display(cached.repos));
                }
                return Err(format!("Network error: {}. Using fallback.", e));
            }
        };

        if !response.ok() {
            if let Ok(cached) = get_cached() {
                return Ok(finalize_repos_for_display(cached.repos));
            }
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let chunk: Vec<Repo> = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                if let Ok(cached) = get_cached() {
                    return Ok(finalize_repos_for_display(cached.repos));
                }
                return Err(format!("Parse error: {}", e));
            }
        };

        if chunk.is_empty() {
            break;
        }
        let chunk_len = chunk.len();
        repos.extend(chunk);
        if chunk_len < 100 {
            break;
        }
    }

    // Merge screenshots from static fallback (API does not return them)
    let fallback = static_fallback();
    let screenshot_map: HashMap<String, String> = fallback
        .into_iter()
        .filter_map(|r| r.screenshot.map(|s| (r.name, s)))
        .collect();
    for repo in &mut repos {
        if let Some(screenshot) = screenshot_map.get(&repo.name) {
            repo.screenshot = Some(screenshot.clone());
        }
    }

    sort_repos_for_display(&mut repos);
    filter_hidden_from_grid(&mut repos);

    set_cache(&repos);
    Ok(repos)
}

/// Returns repos from cache, API, or static fallback. Never fails.
pub async fn fetch_repos_with_fallback() -> Vec<Repo> {
    match fetch_repos().await {
        Ok(repos) => repos,
        Err(_) => finalize_repos_for_display(static_fallback()),
    }
}

/// Reads cached repos from localStorage.
fn get_cached() -> Result<CachedRepos, ()> {
    LocalStorage::get(CACHE_KEY).map_err(|_| ())
}

/// Writes repos to localStorage with current timestamp.
fn set_cache(repos: &[Repo]) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cached = CachedRepos {
        repos: repos.to_vec(),
        fetched_at: now,
    };
    let _ = LocalStorage::set(CACHE_KEY, cached);
}

