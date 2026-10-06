use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, AvatarImage, Item, ItemActions, ItemContent, ItemDescription,
    ItemMedia, ItemTitle,
};
use crate::views::PageHeader;

struct Conversation {
    name: &'static str,
    initials: &'static str,
    last: &'static str,
    time: &'static str,
    unread: usize,
    avatar: &'static str,
    online: bool,
}

#[component]
pub fn Chat() -> Element {
    let router = router();

    let conversations = [
        Conversation {
            name: "Alice Kim",
            initials: "AK",
            last: "The calendar sidebar looks great now!",
            time: "09:44",
            unread: 2,
            avatar: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&auto=format&fit=crop&q=80",
            online: true,
        },
        Conversation {
            name: "Design Team",
            initials: "DT",
            last: "Shipped the new empty states 🎉",
            time: "09:12",
            unread: 0,
            avatar: "https://images.unsplash.com/photo-1522075469751-3a6694fb2f61?w=100&auto=format&fit=crop&q=80",
            online: false,
        },
        Conversation {
            name: "Marc Rivera",
            initials: "MR",
            last: "Can you review my PR when free?",
            time: "Yesterday",
            unread: 1,
            avatar: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&auto=format&fit=crop&q=80",
            online: true,
        },
        Conversation {
            name: "Release Bot",
            initials: "RB",
            last: "v1.0.0-beta.2 published to staging",
            time: "Monday",
            unread: 0,
            avatar: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80",
            online: false,
        },
    ];

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Chat",
                description: "Recent conversations across your workspace.",
            }

            div { class: "flex flex-col gap-2",
                for conversation in conversations.iter() {
                    {
                        let name = conversation.name;
                        rsx! {
                            Item {
                                key: "{name}",
                                class: "border-border/40",
                                onclick: move |_| {
                                    let _ = router.push("/chat/messaging");
                                },
                                ItemMedia { class: "bg-transparent p-0",
                                    div { class: "relative",
                                        Avatar { class: "size-10 rounded-lg overflow-hidden border border-border/40",
                                            AvatarImage {
                                                src: conversation.avatar.to_string(),
                                                alt: name.to_string(),
                                            }
                                            AvatarFallback { "{conversation.initials}" }
                                        }
                                        if conversation.online {
                                            span { class: "absolute -bottom-0.5 -right-0.5 size-2.5 rounded-full border-2 border-card bg-primary" }
                                        }
                                    }
                                }
                                ItemContent {
                                    ItemTitle { "{conversation.name}" }
                                    ItemDescription { "{conversation.last}" }
                                }
                                ItemActions {
                                    div { class: "flex flex-col items-end gap-1",
                                        span { class: "text-[10px] text-muted-foreground",
                                            "{conversation.time}"
                                        }
                                        if conversation.unread > 0 {
                                            span { class: "flex h-4 min-w-4 items-center justify-center rounded-full bg-primary px-1 text-[10px] font-medium text-primary-foreground",
                                                "{conversation.unread}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
