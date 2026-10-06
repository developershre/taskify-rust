use crate::components::ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use dioxus::prelude::*;

fn capitalize(segment: &str) -> String {
    let mut chars = segment.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[component]
pub fn BreadcrumbComponent() -> Element {
    let router = router();
    let path = router.full_route_string();

    let segments: Vec<&str> = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();

    let segments: Vec<String> = if segments.is_empty() {
        vec!["dashboard".to_string()]
    } else {
        segments
            .into_iter()
            .map(|segment| segment.to_string())
            .collect()
    };

    let total = segments.len();
    let mut current_path = String::new();

    let breadcrumb_items: Vec<Element> = segments
        .iter()
        .enumerate()
        .map(|(index, segment)| {
            current_path.push('/');
            current_path.push_str(segment);

            let href = current_path.clone();
            let route = href.clone();
            let label = capitalize(segment);
            let is_last = index + 1 == total;

            rsx! {
                BreadcrumbItem {
                    key: "{href}-item",
                    if is_last {
                        BreadcrumbPage { "{label}" }
                    } else {
                        BreadcrumbLink {
                            href: href.clone(),
                            onclick: move |event: MouseEvent| {
                                event.prevent_default();
                                let _ = router.push(route.clone());
                            },
                            "{label}"
                        }
                    }
                }
                if !is_last {
                    BreadcrumbSeparator { key: "{href}-separator" }
                }
            }
        })
        .collect();

    rsx! {
        Breadcrumb {
            BreadcrumbList {
                {breadcrumb_items.into_iter()}
            }
        }
    }
}
