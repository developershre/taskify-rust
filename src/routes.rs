use dioxus::prelude::*;

use crate::components::{AppFrame, TitleBar};
use crate::views::{
    Analytics, Calendar, Chat, ChatIssues, ChatMail, ChatMessaging, ComponentsCalendar, Home, Mcp,
    Projects, ProjectsGithub, ProjectsPersonal, Settings, Tasks, TasksAll, TasksArchived,
    TasksUrgent,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/")]
    Home {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/tasks", Tasks)]
    Tasks {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/tasks/all", TasksAll)]
    TasksAll {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/tasks/archived", TasksArchived)]
    TasksArchived {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/tasks/urgent", TasksUrgent)]
    TasksUrgent {},

    #[route("/projects", Projects)]
    Projects {},
    #[route("/projects/personal", ProjectsPersonal)]
    ProjectsPersonal {},
    #[route("/projects/github", ProjectsGithub)]
    ProjectsGithub {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/calendar", Calendar)]
    Calendar {},

    #[route("/chat", Chat)]
    Chat {},
    #[route("/chat/messaging", ChatMessaging)]
    ChatMessaging {},
    #[route("/chat/mail", ChatMail)]
    ChatMail {},
    #[route("/chat/issues", ChatIssues)]
    ChatIssues {},

    #[route("/analytics", Analytics)]
    Analytics {},

    #[route("/mcp", Mcp)]
    Mcp {},

    #[route("/settings", Settings)]
    Settings {},

    #[layout(TitleBar)]
    #[layout(AppFrame)]
    #[route("/components/calendar")]
    ComponentsCalendar {},
}
