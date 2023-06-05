use yew::prelude::*;

const STROKE: AttrValue = AttrValue::Static("currentColor");
const STROKE_WIDTH: AttrValue = AttrValue::Static("1.5");
const LEN: AttrValue = AttrValue::Static("32");
const BOX: AttrValue = AttrValue::Static("0 0 24 24");
const FILL: AttrValue = AttrValue::Static("none");
const LCAP: AttrValue = AttrValue::Static("round");
const LJOIN: AttrValue = AttrValue::Static("round");

#[function_component(ChooseViewIcon)]
pub fn choose_view_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
        stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8"/>
            <path d="M10 19v-3.96 3.15"></path><path d="M7 19h5"/>
            <rect width="6" height="10" x="16" y="12" rx="2"/>
        </svg>
    }
}

#[function_component(PreviewEnabledIcon)]
pub fn preview_enabled_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
        stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/>
            <circle cx="12" cy="12" r="3"/>
        </svg>
    }
}

#[function_component(PreviewDisabledIcon)]
pub fn preview_disabled_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
        stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/>
            <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/>
            <path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/>
            <line x1="2" x2="22" y1="2" y2="22"/>
        </svg>
    }
}

#[function_component(PlusIcon)]
pub fn plus_icon() -> Html {
    html! {
        <svg width={20} height={20} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
        stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <line x1="12" x2="12" y1="5" y2="19"/>
            <line x1="5" x2="19" y1="12" y2="12"/>
        </svg>
    }
}

#[function_component(MinusIcon)]
pub fn minus_icon() -> Html {
    html! {
        <svg width={20} height={20} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
        stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <line x1="5" x2="19" y1="12" y2="12"/>
        </svg>
    }
}

#[function_component(AlbumIcon)]
pub fn album_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <circle cx="12" cy="12" r="10"></circle>
            <circle cx="12" cy="12" r="3"></circle>
        </svg>
    }
}

#[function_component(FolderIcon)]
pub fn folder_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M17.5 22h.5c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3">
            </path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <path d="M10 20v-1a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0Z"></path>
            <path d="M6 20v-1a2 2 0 1 0-4 0v1a2 2 0 1 0 4 0Z"></path>
            <path d="M2 19v-3a6 6 0 0 1 12 0v3"></path>
        </svg>
    }
}

#[function_component(PlaylistIcon)]
pub fn playlist_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M21 15V6"></path>
            <path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
            <path d="M12 12H3"></path>
            <path d="M16 6H3"></path>
            <path d="M12 18H3"></path>
        </svg>
    }
}

#[function_component(SettingsIcon)]
pub fn settings_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M20 7h-9"></path>
            <path d="M14 17H5"></path>
            <circle cx="17" cy="17" r="3"></circle>
            <circle cx="7" cy="7" r="3"></circle>
        </svg>
    }
}

#[function_component(AddFileIcon)]
pub fn add_file_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <path d="M3 15h6"></path>
            <path d="M6 12v6"></path>
        </svg>
    }
}

#[function_component(SaveIcon)]
pub fn save_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
        </svg>
    }
}

#[function_component(FolderAddIcon)]
pub fn folder_add_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path
                d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z">
            </path>
            <line x1="12" y1="10" x2="12" y2="16"></line>
            <line x1="9" y1="13" x2="15" y2="13"></line>
        </svg>
    }
}

#[function_component(PlaylistAddIcon)]
pub fn playlist_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M11 12H3"></path>
            <path d="M16 6H3"></path>
            <path d="M16 18H3"></path>
            <path d="M18 9v6"></path>
            <path d="M21 12h-6"></path>
        </svg>
    }
}

#[function_component(UndoIcon)]
pub fn undo_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="M9 14 4 9l5-5"></path><path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11"></path>
        </svg>
    }
}

#[function_component(RedoIcon)]
pub fn redo_icon() -> Html {
    html! {
        <svg width={LEN} height={LEN} viewBox={BOX} fill={FILL} stroke={STROKE} stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} stroke-linejoin={LJOIN}>
            <path d="m15 14 5-5-5-5"></path><path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13"></path>
        </svg>
    }
}