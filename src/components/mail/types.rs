#[derive(Clone, PartialEq, Debug)]
pub struct MailLabel {
    pub name: &'static str,
    pub dark: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Mail {
    pub id: String,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub preview: String,
    pub body: Vec<String>,
    pub date: String,
    pub read: bool,
    pub online: bool,
    pub is_starred: bool,
    pub folder: String,
    pub labels: Vec<MailLabel>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MailFolder {
    pub id: &'static str,
    pub name: &'static str,
    pub count: Option<usize>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MailLabelFolder {
    pub name: &'static str,
    pub count: usize,
    pub color: &'static str,
}

#[allow(dead_code)]
pub fn sample_folders() -> Vec<MailFolder> {
    vec![
        MailFolder {
            id: "inbox",
            name: "Inbox",
            count: Some(128),
        },
        MailFolder {
            id: "drafts",
            name: "Drafts",
            count: Some(9),
        },
        MailFolder {
            id: "sent",
            name: "Sent",
            count: None,
        },
        MailFolder {
            id: "junk",
            name: "Junk",
            count: Some(23),
        },
        MailFolder {
            id: "trash",
            name: "Trash",
            count: None,
        },
        MailFolder {
            id: "archive",
            name: "Archive",
            count: None,
        },
    ]
}

pub fn sample_label_folders() -> Vec<MailLabelFolder> {
    vec![
        MailLabelFolder {
            name: "Social",
            count: 972,
            color: "#8b5cf6",
        },
        MailLabelFolder {
            name: "Updates",
            count: 342,
            color: "#14b8a6",
        },
        MailLabelFolder {
            name: "Forums",
            count: 128,
            color: "#f97316",
        },
        MailLabelFolder {
            name: "Shopping",
            count: 8,
            color: "#84cc16",
        },
        MailLabelFolder {
            name: "Promotions",
            count: 21,
            color: "#ec4899",
        },
    ]
}

pub fn sample_mails() -> Vec<Mail> {
    vec![
        Mail {
            id: "william".to_string(),
            name: "William Smith".to_string(),
            email: "william.smith@example.com".to_string(),
            subject: "Meeting Tomorrow".to_string(),
            preview: "Hi, let's have a meeting tomorrow to discuss the project. I've been reviewing the project details and hav..."
                .to_string(),
            body: vec![
                "Hi, let's have a meeting tomorrow to discuss the project. I've been reviewing the project details and have a few questions I'd like to go over with you.".to_string(),
                "I think it would be helpful if we could align on the milestones for the next sprint and clarify the scope of the design review before we hand anything off to engineering.".to_string(),
                "Let me know what time works best for you — I'm free most of the afternoon.".to_string(),
            ],
            date: "Today, 10:30 AM".to_string(),
            read: false,
            online: true,
            is_starred: true,
            folder: "inbox".to_string(),
            labels: vec![
                MailLabel {
                    name: "meeting",
                    dark: false,
                },
                MailLabel {
                    name: "work",
                    dark: true,
                },
                MailLabel {
                    name: "important",
                    dark: false,
                },
            ],
        },
        Mail {
            id: "alice".to_string(),
            name: "Alice Smith".to_string(),
            email: "alice.smith@example.com".to_string(),
            subject: "Re: Project Update".to_string(),
            preview: "Thank you for the project update. It looks great! I've gone through the report, and the progress is..."
                .to_string(),
            body: vec![
                "Thank you for the project update. It looks great! I've gone through the report, and the progress is impressive — the team has done an excellent job hitting the targets we set last quarter.".to_string(),
                "I've left a couple of notes in the attached document, mostly around the timeline for phase two. Happy to discuss further whenever you're free.".to_string(),
            ],
            date: "Yesterday, 4:15 PM".to_string(),
            read: false,
            online: false,
            is_starred: false,
            folder: "inbox".to_string(),
            labels: vec![
                MailLabel {
                    name: "work",
                    dark: true,
                },
                MailLabel {
                    name: "important",
                    dark: false,
                },
            ],
        },
        Mail {
            id: "bob".to_string(),
            name: "Bob Johnson".to_string(),
            email: "bob.johnson@example.com".to_string(),
            subject: "Weekend Plans".to_string(),
            preview: "Any plans for the weekend? I was thinking of going hiking in the nearby mountains. It's been a while since..."
                .to_string(),
            body: vec![
                "Any plans for the weekend? I was thinking of going hiking in the nearby mountains. It's been a while since we last caught up properly, and it would be great to get out of the city for a bit.".to_string(),
                "Let me know if you're in — I can drive, and we can grab breakfast on the way out.".to_string(),
            ],
            date: "2 days ago".to_string(),
            read: true,
            online: false,
            is_starred: false,
            folder: "inbox".to_string(),
            labels: vec![MailLabel {
                name: "personal",
                dark: false,
            }],
        },
        Mail {
            id: "emily".to_string(),
            name: "Emily Davis".to_string(),
            email: "emily.davis@example.com".to_string(),
            subject: "Re: Question about Budget".to_string(),
            preview: "I have a question about the budget for the upcoming quarter. Specifically, I'd like to understand..."
                .to_string(),
            body: vec![
                "I have a question about the budget for the upcoming quarter. Specifically, I'd like to understand how much flexibility we have in the design tooling line item before we finalize the plan.".to_string(),
                "Could we chat briefly tomorrow morning? I should be at my desk from nine.".to_string(),
            ],
            date: "3 days ago".to_string(),
            read: false,
            online: true,
            is_starred: true,
            folder: "inbox".to_string(),
            labels: vec![MailLabel {
                name: "work",
                dark: true,
            }],
        },
        Mail {
            id: "lucas".to_string(),
            name: "Lucas Johnson".to_string(),
            email: "lucas.johnson@example.com".to_string(),
            subject: "Lunch Today?".to_string(),
            preview: "Want to grab lunch today? I heard the new place around the corner does incredible sandwiches…"
                .to_string(),
            body: vec![
                "Want to grab lunch today? I heard the new place around the corner does incredible sandwiches, and I've been wanting to try it.".to_string(),
                "I'm flexible between noon and two — just let me know what works for you.".to_string(),
            ],
            date: "4 days ago".to_string(),
            read: true,
            online: false,
            is_starred: false,
            folder: "inbox".to_string(),
            labels: vec![MailLabel {
                name: "personal",
                dark: false,
            }],
        },
        Mail {
            id: "olivia".to_string(),
            name: "Olivia Wilson".to_string(),
            email: "olivia.wilson@example.com".to_string(),
            subject: "Re: Team Offsite".to_string(),
            preview: "The venue is confirmed for the 14th. I've attached the agenda — take a look before…"
                .to_string(),
            body: vec![
                "The venue is confirmed for the 14th. I've attached the agenda — take a look before Friday so we can finalize the session timings.".to_string(),
                "Catering will arrive at 9am, and the main hall is booked for the whole day.".to_string(),
            ],
            date: "1 week ago".to_string(),
            read: true,
            online: false,
            is_starred: false,
            folder: "archive".to_string(),
            labels: vec![
                MailLabel {
                    name: "meeting",
                    dark: false,
                },
                MailLabel {
                    name: "personal",
                    dark: false,
                },
            ],
        },
        Mail {
            id: "ava".to_string(),
            name: "Ava Martinez".to_string(),
            email: "ava.martinez@example.com".to_string(),
            subject: "Design Review Notes".to_string(),
            preview: "Here are my notes from yesterday's review. Overall the new flows look solid, just a…"
                .to_string(),
            body: vec![
                "Here are my notes from yesterday's review. Overall the new flows look solid, just a few edge cases around empty states and error handling we should cover before the next milestone.".to_string(),
                "I'll prepare the updated prototypes for Monday.".to_string(),
            ],
            date: "2 weeks ago".to_string(),
            read: true,
            online: false,
            is_starred: true,
            folder: "inbox".to_string(),
            labels: vec![MailLabel {
                name: "important",
                dark: false,
            }],
        },
        Mail {
            id: "noah".to_string(),
            name: "Noah Brown".to_string(),
            email: "noah.brown@example.com".to_string(),
            subject: "Re: Budget Approval".to_string(),
            preview: "Good news — the budget was approved! We can start allocating resources to the…"
                .to_string(),
            body: vec![
                "Good news — the budget was approved! We can start allocating resources to the new initiatives right away.".to_string(),
                "I'll send over the breakdown by end of day so everyone has visibility.".to_string(),
            ],
            date: "3 weeks ago".to_string(),
            read: true,
            online: false,
            is_starred: false,
            folder: "archive".to_string(),
            labels: vec![MailLabel {
                name: "work",
                dark: true,
            }],
        },
    ]
}
