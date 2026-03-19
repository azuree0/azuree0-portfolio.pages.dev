//! Injects `<link rel="prefetch">` for likely next navigation (GitHub repo pages).

/// Adds or replaces a prefetch hint in `<head>` for the given URL (e.g. GitHub repo).
pub fn prefetch_navigation_url(href: &str) {
    const LINK_ID: &str = "portfolio-prefetch-next";
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(head) = document.head() else {
        return;
    };
    if let Some(old) = document.get_element_by_id(LINK_ID) {
        let _ = old.remove();
    }
    let Ok(link) = document.create_element("link") else {
        return;
    };
    let _ = link.set_attribute("rel", "prefetch");
    let _ = link.set_attribute("href", href);
    let _ = link.set_id(LINK_ID);
    let _ = head.append_child(&link);
}
