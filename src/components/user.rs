use dioxus::prelude::*;

use crate::components::ui::{
    Avatar, AvatarFallback, AvatarImage, DropdownMenu, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuTrigger,
};

#[component]
pub fn User() -> Element {
    rsx! {
       DropdownMenu {
           DropdownMenuTrigger {
               children: rsx! {
                   Avatar{
                       class: "size-4",
                       AvatarImage{
                           src: "https://ui.shadcn.com/avatars/shadcn.jpg",
                           alt: "",
                       }
                       AvatarFallback{
                           children: rsx! {
                               span { "CN" }
                           },
                       }
                   }
               },
           },
           DropdownMenuContent {
               children: rsx! {
                   DropdownMenuLabel {
                       children: rsx! {

                       },
                   }
                   DropdownMenuItem {
                       children: rsx! {
                       },
                   }
               },
           }
       }
    }
}
