use dioxus::prelude::*;

use crate::components::ui::{Badge, Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, Switch};
use crate::views::PageHeader;

struct McpServer {
    name: &'static str,
    description: &'static str,
    status: &'static str,
    tools: usize,
    default_on: bool,
}

#[component]
pub fn Mcp() -> Element {
    let servers = [
        McpServer {
            name: "Filesystem",
            description: "Read and write files on your local machine",
            status: "Connected",
            tools: 12,
            default_on: true,
        },
        McpServer {
            name: "GitHub",
            description: "Manage issues, pull requests and repositories",
            status: "Connected",
            tools: 24,
            default_on: true,
        },
        McpServer {
            name: "Postgres",
            description: "Query your PostgreSQL database",
            status: "Disabled",
            tools: 8,
            default_on: false,
        },
        McpServer {
            name: "Playwright",
            description: "Browser automation and end-to-end testing",
            status: "Disconnected",
            tools: 15,
            default_on: false,
        },
    ];

    rsx! {
        div { class: "flex flex-col",
            PageHeader {
                title: "Mcp",
                description: "Model Context Protocol servers connected to Taskify.",
                actions: rsx! {
                    div { class: "flex items-center gap-2 pt-2",
                        Badge { variant: "secondary", "2 connected" }
                    }
                },
            }

            div { class: "flex flex-col gap-2",
                for server in servers.iter() {
                    {
                        let status_variant = match server.status {
                            "Connected" => "default",
                            "Disabled" => "outline",
                            _ => "secondary",
                        };

                        rsx! {
                            Item {
                                key: "{server.name}",
                                class: "border-border/40",
                                ItemMedia {
                                    class: "bg-secondary text-foreground",
                                    svg {
                                        class: "size-5",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M12 8V4H8" }
                                        rect {
                                            width: "16",
                                            height: "12",
                                            x: "4",
                                            y: "8",
                                            rx: "2",
                                        }
                                        path { d: "M2 14h2M20 14h2M15 13v2M9 13v2" }
                                    }
                                }
                                ItemContent {
                                    ItemTitle { "{server.name}" }
                                    ItemDescription { "{server.description}" }
                                }
                                ItemActions {
                                    Badge {
                                        variant: "outline",
                                        size: "sm",
                                        "{server.tools} tools"
                                    }
                                    Badge {
                                        variant: status_variant,
                                        size: "sm",
                                        "{server.status}"
                                    }
                                    Switch {
                                        default_checked: server.default_on,
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
