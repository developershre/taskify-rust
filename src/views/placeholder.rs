use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn PlaceholderPage() -> Element {
    let title = match router().current::<Route>() {
        Route::Home {} => "Dashboard",
        Route::Tasks {} => "Tasks",
        Route::TasksAll {} => "All Tasks",
        Route::TasksArchived {} => "Archived",
        Route::TasksUrgent {} => "Urgent",
        Route::Projects {} => "Projects",
        Route::ProjectsPersonal {} => "Personal",
        Route::ProjectsGithub {} => "Github Repos",
        Route::Calendar {} => "Calendar",
        Route::Chat {} => "Chat",
        Route::ChatMessaging {} => "Messaging",
        Route::ChatMail {} => "Mail",
        Route::ChatIssues {} => "Issues",
        Route::Analytics {} => "Analytics",
        Route::Mcp {} => "Mcp",
    };

    rsx! {
        div { class: "flex flex-col gap-1 pt-4",
            h1 { class: "text-2xl sm:text-3xl font-bold tracking-tight text-foreground",
                "{title}"
            }
            p { class: "text-sm text-muted-foreground",
                "This page is under construction."
            }
        }
    }
}
