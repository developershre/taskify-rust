use dioxus::prelude::*;

// ============================================================
// Context
// ============================================================

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct TooltipContext {
    pub open: Signal<bool>,
}

// ============================================================
// Tooltip Provider
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProviderProps {
    pub children: Element,
}

#[component]
pub fn TooltipProvider(props: TooltipProviderProps) -> Element {
    rsx! {
        div {
            "data-slot": "tooltip-provider",
            class: "contents",
            {props.children}
        }
    }
}

// ============================================================
// Tooltip Root
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    #[props(default)]
    pub open: Option<Signal<bool>>,

    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let local_open = use_signal(|| false);
    let open = props.open.unwrap_or(local_open);

    use_context_provider(|| TooltipContext { open });

    rsx! {
        div {
            "data-slot": "tooltip",
            class: "relative inline-flex",
            {props.children}
        }
    }
}

// ============================================================
// Tooltip Trigger
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    #[props(default)]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let mut ctx = use_context::<TooltipContext>();

    rsx! {
        div {
            "data-slot": "tooltip-trigger",
            class: "inline-flex cursor-pointer {props.class}",
            onmouseenter: move |_| {
                ctx.open.set(true);
            },
            onmouseleave: move |_| {
                ctx.open.set(false);
            },
            onfocus: move |_| {
                ctx.open.set(true);
            },
            onblur: move |_| {
                ctx.open.set(false);
            },
            {props.children}
        }
    }
}

// ============================================================
// Tooltip Content
// ============================================================

#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    #[props(default = "top".to_string())]
    pub side: String,

    #[props(default)]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx = use_context::<TooltipContext>();

    if !*ctx.open.read() {
        return rsx! {};
    }

    let pos_class = match props.side.as_str() {
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-1.5",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-1.5",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-1.5",
        _ => "bottom-full left-1/2 -translate-x-1/2 mb-1.5",
    };

    let class = format!(
        "absolute z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs \
         text-primary-foreground shadow-md animate-in fade-in-0 zoom-in-95 pointer-events-none \
         whitespace-nowrap select-none {} {}",
        pos_class, props.class
    );

    rsx! {
        div {
            "data-slot": "tooltip-content",
            role: "tooltip",
            class: class,
            {props.children}
        }
    }
}
