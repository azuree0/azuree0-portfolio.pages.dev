use crate::models::Repo;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const CACHE_KEY: &str = "portfolio_repos";
const CACHE_TTL_SECS: u64 = 900; // 15 min

#[derive(Serialize, Deserialize)]
struct CachedRepos {
    repos: Vec<Repo>,
    fetched_at: u64,
}

/// Static fallback repos when API fails (azuree0's known repos)
pub fn static_fallback() -> Vec<Repo> {
    let base = "https://github.com/azuree0";
    vec![
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
            name: "Latrones".to_string(),
            description: Some("Roman board game".to_string()),
            html_url: format!("{}/Latrones", base),
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/182fd35b-a924-4749-8f37-9f48060ec49f".to_string()),
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
            name: "Go".to_string(),
            description: Some("Go board game".to_string()),
            html_url: format!("{}/Go", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/f52cbdc4-afe9-4169-8a03-ed025b6a834a".to_string()),
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
            name: "Mehen".to_string(),
            description: Some("Ancient Egyptian snake game".to_string()),
            html_url: format!("{}/Mehen", base),
            language: Some("Rust".to_string()),
            stargazers_count: 1,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/b9a324c1-822d-49ed-b88e-13fbc2b17f04".to_string()),
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
            name: "Silent-data-corruption".to_string(),
            description: Some("C++ project".to_string()),
            html_url: format!("{}/Silent-data-corruption", base),
            language: Some("C++".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/1db52073-faf9-4e6d-895c-36c66dc1625d".to_string()),
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
            name: "Liquid".to_string(),
            description: Some("Liquid templates".to_string()),
            html_url: format!("{}/Liquid", base),
            language: Some("Liquid".to_string()),
            stargazers_count: 0,
            updated_at: String::new(),
            screenshot: Some("https://github.com/user-attachments/assets/ac0f0af2-e95e-4591-b848-e30c89675822".to_string()),
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
    ]
}

pub async fn fetch_repos() -> Result<Vec<Repo>, String> {
    // Try cache first (instant load if valid)
    if let Ok(cached) = get_cached() {
        if !is_stale(cached.fetched_at) {
            return Ok(cached.repos);
        }
    }

    let url = "https://api.github.com/users/azuree0/repos?sort=updated&per_page=100";
    let response = match gloo_net::http::Request::get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            // Network/CORS failure: try cache, then static fallback
            if let Ok(cached) = get_cached() {
                return Ok(cached.repos);
            }
            return Err(format!("Network error: {}. Using fallback.", e));
        }
    };

    if !response.ok() {
        if let Ok(cached) = get_cached() {
            return Ok(cached.repos);
        }
        return Err(format!("GitHub API error: {}", response.status()));
    }

    let mut repos: Vec<Repo> = match response.json().await {
        Ok(r) => r,
        Err(e) => {
            if let Ok(cached) = get_cached() {
                return Ok(cached.repos);
            }
            return Err(format!("Parse error: {}", e));
        }
    };

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

    set_cache(&repos);
    Ok(repos)
}

/// Returns repos from cache, API, or static fallback. Never fails.
pub async fn fetch_repos_with_fallback() -> Vec<Repo> {
    match fetch_repos().await {
        Ok(repos) => repos,
        Err(_) => static_fallback(),
    }
}

fn get_cached() -> Result<CachedRepos, ()> {
    LocalStorage::get(CACHE_KEY).map_err(|_| ())
}

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

fn is_stale(fetched_at: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now.saturating_sub(fetched_at) > CACHE_TTL_SECS
}
