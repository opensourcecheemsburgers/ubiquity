/// UI Icons from https://lucide.dev

use yew::prelude::*;

const STROKE: AttrValue = AttrValue::Static("currentColor");
const STROKE_WIDTH: AttrValue = AttrValue::Static("1.5");
const BOX: AttrValue = AttrValue::Static("0 0 24 24");
const FILL: AttrValue = AttrValue::Static("none");
const LCAP: AttrValue = AttrValue::Static("round");
const LJOIN: AttrValue = AttrValue::Static("round");

#[derive(Debug, PartialEq, Properties)]
pub struct SvgProps {
    #[prop_or(AttrValue::from("24"))]
    pub size: AttrValue,
    pub children: Children
}

#[function_component(Svg)]
pub fn svg(props: &SvgProps) -> Html {
    html! {
        <svg 
            width={&props.size} 
            height={&props.size} 
            viewBox={BOX} 
            fill={FILL} 
            stroke={STROKE} 
            stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP} 
            stroke-linejoin={LJOIN}>
            { props.children.clone() }
        </svg>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or(AttrValue::from("24"))]
    pub size: AttrValue,
}

#[function_component(ChooseViewIcon)]
pub fn choose_view_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8"/>
            <path d="M10 19v-3.96 3.15"/>
            <path d="M7 19h5"/>
            <rect width="6" height="10" x="16" y="12" rx="2"/>
        </Svg>
    }
}

#[function_component(PreviewEnabledIcon)]
pub fn preview_enabled_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/>
            <circle cx="12" cy="12" r="3"/>
        </Svg>
    }
}

#[function_component(PreviewDisabledIcon)]
pub fn preview_disabled_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-keyboard"><rect width="20" height="16" x="2" y="4" rx="2" ry="2"/><path d="M6 8h.001"/><path d="M10 8h.001"/><path d="M14 8h.001"/><path d="M18 8h.001"/><path d="M8 12h.001"/><path d="M12 12h.001"/><path d="M16 12h.001"/><path d="M7 16h10"/></svg>
        </Svg>
    }
}

#[function_component(PlusIcon)]
pub fn plus_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <line x1="12" x2="12" y1="5" y2="19"/>
            <line x1="5" x2="19" y1="12" y2="12"/>
        </Svg>
    }
}

#[function_component(MinusIcon)]
pub fn minus_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <line x1="5" x2="19" y1="12" y2="12"/>
        </Svg>
    }
}

#[function_component(HamburgerIcon)]
pub fn hamburger_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <line x1="4" x2="20" y1="12" y2="12"/>
            <line x1="4" x2="20" y1="6" y2="6"/>
            <line x1="4" x2="20" y1="18" y2="18"/>
        </Svg>
    }
}

#[function_component(WrenchIcon)]
pub fn wrench_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
        </Svg>

    }
}

#[function_component(AlbumIcon)]
pub fn album_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <circle cx="12" cy="12" r="10"/>
            <circle cx="12" cy="12" r="3"/>
        </Svg>
    }
}

#[function_component(FolderIcon)]
pub fn folder_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M17.5 22h.5c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3">
            </path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <path d="M10 20v-1a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0Z"></path>
            <path d="M6 20v-1a2 2 0 1 0-4 0v1a2 2 0 1 0 4 0Z"></path>
            <path d="M2 19v-3a6 6 0 0 1 12 0v3"></path>
        </Svg>
    }
}

#[function_component(PlaylistIcon)]
pub fn playlist_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M21 15V6"></path>
            <path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
            <path d="M12 12H3"></path>
            <path d="M16 6H3"></path>
            <path d="M12 18H3"></path>
        </Svg>
    }
}

#[function_component(SettingsIcon)]
pub fn settings_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M20 7h-9"></path>
            <path d="M14 17H5"></path>
            <circle cx="17" cy="17" r="3"></circle>
            <circle cx="7" cy="7" r="3"></circle>
        </Svg>
    }
}

#[function_component(AddFileIcon)]
pub fn add_file_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <path d="M3 15h6"></path>
            <path d="M6 12v6"></path>
        </Svg>
    }
}

#[function_component(SaveIcon)]
pub fn save_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
        </Svg>
    }
}

#[function_component(FolderAddIcon)]
pub fn folder_add_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path
                d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z">
            </path>
            <line x1="12" y1="10" x2="12" y2="16"></line>
            <line x1="9" y1="13" x2="15" y2="13"></line>
        </Svg>
    }
}

#[function_component(PlaylistAddIcon)]
pub fn playlist_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M11 12H3"></path>
            <path d="M16 6H3"></path>
            <path d="M16 18H3"></path>
            <path d="M18 9v6"></path>
            <path d="M21 12h-6"></path>
        </Svg>
    }
}

#[function_component(UndoIcon)]
pub fn undo_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M9 14 4 9l5-5"></path><path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11"></path>
        </Svg>
    }
}

#[function_component(RedoIcon)]
pub fn redo_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="m15 14 5-5-5-5"></path><path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13"></path>
        </Svg>
    }
}

#[function_component(BoldIcon)]
pub fn bold_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M14 12a4 4 0 0 0 0-8H6v8"/>
            <path d="M15 20a4 4 0 0 0 0-8H6v8Z"/>
        </Svg>
    }
}

#[function_component(ItalicsIcon)]
pub fn bold_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/>
            <line x1="15" x2="9" y1="4" y2="20"/>
        </Svg>
    }
}

#[function_component(QuoteIcon)]
pub fn quote_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"/>
            <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"/>
        </Svg>
    }
}

#[function_component(HeadingIcon)]
pub fn heading_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M6 12h12"/>
            <path d="M6 20V4"/>
            <path d="M18 20V4"/>
        </Svg>
    }
}

#[function_component(Heading1Icon)]
pub fn heading1_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><path d="m17 12 3-2v8"/>
        </Svg>
    }
}
#[function_component(Heading2Icon)]
pub fn heading2_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/>
            <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"/>
        </Svg>
    }
}
#[function_component(Heading3Icon)]
pub fn heading3_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/>
            <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"/>
            <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"/>
        </Svg>
    }
}
#[function_component(Heading4Icon)]
pub fn heading4_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/>
            <path d="M17 10v4h4"/><path d="M21 10v8"/>
        </Svg>
    }
}
#[function_component(Heading5Icon)]
pub fn heading5_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/>
            <path d="M4 18V6"/>
            <path d="M12 18V6"/>
            <path d="M17 13v-3h4"/>
            <path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"/>
        </Svg>
    }
}
#[function_component(Heading6Icon)]
pub fn heading6_icon(props: &IconProps) -> Html {
    html! {
        <Svg size={&props.size}>
            <path d="M4 12h8"/>
            <path d="M4 18V6"/>
            <path d="M12 18V6"/>
            <circle cx="19" cy="16" r="2"/>
            <path d="M20 10c-2 2-3 3.5-3 6"/>
        </Svg>
    }
}

#[function_component(MoneroQr)]
pub fn monero_qr() -> Html {
    // (https://fukuchi.org/works/qrencode/index.html)

    html! {
        <svg class="h-64 lg:h-96 w-64 lg:w-96" viewBox="0 0 61 61" preserveAspectRatio="none" version="1.1" xmlns="http://www.w3.org/2000/svg">
            <g id="QRcode">
                <rect x="0" y="0"/>
                <path class="stroke-base-content" transform="translate(4,4.5)" d="M0,0h7M8,0h1M12,0h1M15,0h1M17,0h2M20,0h1M23,0h1M25,0h1M27,0h1M30,0h1M35,0h3M40,0h3M46,0h7M0,1h1M6,1h1M8,1h2M14,1h2M17,1h1M19,1h1M22,1h5M28,1h1M31,1h1M33,1h1M38,1h1M41,1h3M46,1h1M52,1h1M0,2h1M2,2h3M6,2h1M8,2h2M11,2h1M23,2h1M26,2h2M30,2h5M36,2h1M38,2h1M40,2h1M43,2h1M46,2h1M48,2h3M52,2h1M0,3h1M2,3h3M6,3h1M12,3h1M14,3h2M17,3h1M19,3h1M22,3h1M26,3h1M30,3h1M32,3h2M35,3h1M37,3h1M41,3h2M44,3h1M46,3h1M48,3h3M52,3h1M0,4h1M2,4h3M6,4h1M9,4h1M13,4h3M22,4h1M24,4h7M37,4h1M41,4h2M46,4h1M48,4h3M52,4h1M0,5h1M6,5h1M8,5h1M10,5h2M15,5h3M20,5h1M23,5h2M28,5h1M31,5h1M34,5h2M38,5h1M40,5h1M42,5h1M46,5h1M52,5h1M0,6h7M8,6h1M10,6h1M12,6h1M14,6h1M16,6h1M18,6h1M20,6h1M22,6h1M24,6h1M26,6h1M28,6h1M30,6h1M32,6h1M34,6h1M36,6h1M38,6h1M40,6h1M42,6h1M44,6h1M46,6h7M8,7h1M10,7h2M16,7h1M20,7h1M22,7h3M28,7h4M34,7h1M39,7h1M42,7h2M2,8h3M6,8h1M8,8h5M14,8h2M17,8h1M20,8h1M23,8h7M31,8h1M33,8h1M35,8h4M41,8h1M43,8h5M50,8h3M0,9h1M7,9h1M9,9h1M11,9h1M14,9h2M17,9h1M19,9h5M25,9h3M29,9h3M33,9h1M35,9h3M42,9h2M47,9h1M49,9h1M52,9h1M0,10h1M6,10h1M15,10h2M19,10h2M22,10h1M25,10h2M28,10h1M33,10h1M35,10h2M38,10h1M41,10h1M47,10h3M51,10h1M0,11h1M4,11h1M8,11h1M13,11h1M16,11h2M20,11h1M23,11h4M29,11h3M33,11h3M37,11h7M45,11h2M48,11h2M51,11h1M1,12h2M6,12h6M14,12h1M16,12h1M20,12h2M26,12h1M28,12h1M33,12h1M35,12h2M45,12h1M47,12h6M0,13h2M4,13h2M7,13h1M9,13h1M11,13h2M14,13h3M21,13h5M27,13h1M31,13h1M35,13h1M37,13h1M39,13h5M47,13h1M49,13h3M1,14h3M5,14h3M12,14h1M14,14h4M21,14h1M26,14h1M29,14h2M34,14h1M38,14h1M40,14h2M44,14h3M48,14h1M50,14h1M1,15h1M3,15h1M5,15h1M9,15h2M12,15h1M14,15h1M17,15h1M21,15h4M26,15h2M29,15h3M34,15h1M37,15h4M42,15h1M45,15h2M49,15h1M51,15h2M0,16h4M6,16h2M9,16h1M11,16h2M15,16h3M21,16h2M24,16h3M31,16h1M33,16h1M35,16h4M44,16h1M47,16h3M51,16h2M4,17h2M7,17h2M10,17h1M15,17h2M19,17h2M23,17h2M26,17h1M30,17h1M33,17h5M41,17h3M48,17h1M52,17h1M3,18h2M6,18h2M9,18h4M16,18h1M20,18h3M26,18h1M28,18h2M34,18h4M39,18h2M43,18h2M46,18h6M0,19h1M2,19h1M5,19h1M7,19h2M12,19h2M15,19h4M21,19h1M23,19h2M26,19h1M28,19h1M30,19h1M32,19h2M35,19h2M38,19h2M41,19h3M45,19h1M48,19h1M52,19h1M3,20h1M6,20h5M13,20h1M15,20h2M18,20h4M24,20h1M27,20h1M33,20h1M36,20h2M41,20h1M44,20h1M46,20h5M52,20h1M0,21h2M4,21h1M7,21h10M23,21h7M31,21h6M42,21h2M47,21h2M51,21h1M1,22h6M10,22h1M14,22h2M18,22h4M23,22h1M28,22h1M32,22h1M34,22h1M36,22h1M39,22h3M43,22h2M46,22h1M48,22h3M5,23h1M9,23h1M14,23h2M17,23h6M24,23h1M28,23h1M30,23h1M34,23h1M36,23h2M42,23h1M44,23h2M52,23h1M0,24h3M4,24h8M13,24h2M18,24h3M23,24h10M35,24h2M41,24h1M44,24h9M0,25h5M8,25h1M11,25h1M14,25h3M19,25h1M21,25h1M24,25h1M28,25h1M30,25h1M33,25h1M36,25h2M39,25h1M42,25h1M44,25h1M48,25h1M50,25h1M1,26h2M4,26h1M6,26h1M8,26h5M14,26h1M16,26h1M18,26h3M22,26h3M26,26h1M28,26h1M31,26h1M34,26h1M41,26h1M43,26h2M46,26h1M48,26h1M1,27h2M4,27h1M8,27h1M12,27h2M15,27h2M18,27h1M22,27h1M24,27h1M28,27h1M33,27h2M37,27h1M42,27h1M44,27h1M48,27h2M51,27h2M1,28h9M11,28h2M18,28h1M21,28h2M24,28h5M31,28h3M35,28h4M41,28h8M50,28h3M2,29h2M7,29h1M10,29h4M17,29h1M19,29h1M22,29h1M24,29h1M26,29h1M29,29h1M31,29h1M34,29h2M37,29h1M42,29h1M45,29h4M50,29h1M52,29h1M0,30h1M2,30h5M12,30h1M14,30h2M20,30h1M22,30h1M24,30h1M26,30h8M35,30h2M40,30h2M47,30h2M1,31h1M3,31h1M5,31h1M7,31h2M10,31h3M15,31h6M22,31h3M28,31h2M31,31h1M33,31h1M39,31h8M0,32h2M5,32h2M9,32h3M13,32h6M21,32h3M25,32h1M28,32h3M32,32h1M34,32h4M41,32h2M45,32h1M48,32h4M1,33h1M8,33h1M12,33h4M18,33h2M21,33h1M24,33h3M28,33h1M31,33h1M33,33h1M35,33h1M39,33h1M42,33h2M46,33h3M51,33h1M0,34h2M3,34h2M6,34h5M14,34h1M18,34h4M27,34h1M29,34h3M35,34h5M41,34h1M50,34h1M1,35h3M8,35h1M12,35h1M15,35h1M17,35h1M19,35h2M26,35h1M29,35h3M33,35h3M37,35h1M39,35h6M46,35h1M48,35h1M51,35h1M0,36h2M4,36h4M9,36h2M15,36h3M20,36h4M27,36h1M30,36h2M35,36h2M46,36h2M49,36h3M0,37h1M3,37h2M7,37h1M9,37h1M11,37h7M24,37h5M30,37h2M33,37h2M37,37h1M42,37h3M46,37h4M52,37h1M1,38h6M9,38h4M14,38h1M17,38h2M21,38h1M24,38h4M29,38h4M34,38h2M39,38h3M45,38h1M50,38h1M0,39h1M2,39h1M4,39h2M7,39h1M13,39h3M18,39h2M26,39h1M29,39h1M31,39h1M33,39h1M35,39h2M39,39h4M44,39h1M46,39h4M51,39h1M2,40h1M5,40h2M9,40h3M13,40h12M27,40h1M31,40h1M33,40h1M35,40h2M38,40h1M41,40h1M46,40h1M48,40h1M50,40h3M3,41h2M8,41h2M13,41h2M17,41h2M20,41h2M24,41h1M26,41h2M29,41h1M32,41h1M35,41h3M41,41h2M45,41h4M0,42h2M3,42h5M11,42h2M14,42h5M22,42h1M24,42h3M28,42h1M31,42h4M36,42h2M39,42h1M41,42h1M43,42h2M48,42h3M1,43h2M7,43h10M18,43h1M22,43h1M24,43h3M28,43h1M33,43h1M38,43h2M41,43h2M44,43h3M49,43h1M51,43h2M3,44h1M6,44h5M13,44h2M17,44h1M20,44h3M24,44h8M35,44h3M44,44h6M51,44h1M8,45h1M10,45h1M17,45h1M19,45h2M22,45h1M24,45h1M28,45h1M32,45h6M39,45h1M41,45h4M48,45h1M51,45h1M0,46h7M9,46h4M15,46h2M18,46h1M24,46h1M26,46h1M28,46h5M35,46h1M38,46h1M42,46h3M46,46h1M48,46h1M0,47h1M6,47h1M11,47h3M15,47h2M18,47h3M24,47h1M28,47h1M30,47h1M32,47h2M36,47h1M42,47h3M48,47h2M51,47h2M0,48h1M2,48h3M6,48h1M8,48h3M13,48h4M18,48h4M24,48h6M33,48h1M35,48h3M41,48h2M44,48h8M0,49h1M2,49h3M6,49h1M8,49h2M11,49h2M15,49h5M21,49h2M24,49h2M28,49h1M32,49h6M39,49h1M41,49h5M47,49h1M0,50h1M2,50h3M6,50h1M8,50h1M11,50h1M13,50h2M16,50h2M20,50h3M24,50h1M27,50h1M29,50h4M36,50h5M42,50h2M46,50h4M0,51h1M6,51h1M9,51h1M13,51h1M15,51h1M18,51h1M20,51h8M29,51h2M34,51h1M36,51h5M42,51h1M47,51h1M51,51h1M0,52h7M10,52h1M12,52h1M14,52h6M21,52h2M25,52h1M27,52h1M31,52h5M38,52h1M41,52h1M43,52h1M45,52h1M47,52h4"/>
            </g>
        </svg>
    }
}

 #[function_component(BitcoinQr)]
 pub fn bitcoin_qr() -> Html {
    // (https://fukuchi.org/works/qrencode/index.html)

    html! {
        <svg class="h-64 lg:h-96 w-64 lg:w-96" viewBox="0 0 45 45" preserveAspectRatio="none" version="1.1"
            xmlns="http://www.w3.org/2000/svg">
            <g id="QRcode">
                <rect x="0" y="0" />
                <path class="stroke-base-content" transform="translate(4,4.5)"
                    d="M0,0h1M1,0h1M2,0h1M3,0h1M4,0h1M5,0h1M6,0h1M8,0h1M11,0h1M12,0h1M14,0h1M16,0h1M19,0h1M20,0h1M21,0h1M22,0h1M25,0h1M26,0h1M28,0h1M30,0h1M31,0h1M32,0h1M33,0h1M34,0h1M35,0h1M36,0h1M0,1h1M6,1h1M8,1h1M9,1h1M11,1h1M14,1h1M15,1h1M17,1h1M18,1h1M20,1h1M21,1h1M22,1h1M23,1h1M27,1h1M28,1h1M30,1h1M36,1h1M0,2h1M2,2h1M3,2h1M4,2h1M6,2h1M8,2h1M11,2h1M12,2h1M15,2h1M17,2h1M18,2h1M30,2h1M32,2h1M33,2h1M34,2h1M36,2h1M0,3h1M2,3h1M3,3h1M4,3h1M6,3h1M11,3h1M13,3h1M14,3h1M15,3h1M18,3h1M19,3h1M20,3h1M21,3h1M22,3h1M23,3h1M25,3h1M28,3h1M30,3h1M32,3h1M33,3h1M34,3h1M36,3h1M0,4h1M2,4h1M3,4h1M4,4h1M6,4h1M12,4h1M14,4h1M15,4h1M16,4h1M24,4h1M25,4h1M26,4h1M30,4h1M32,4h1M33,4h1M34,4h1M36,4h1M0,5h1M6,5h1M8,5h1M9,5h1M10,5h1M11,5h1M12,5h1M13,5h1M15,5h1M19,5h1M21,5h1M22,5h1M23,5h1M24,5h1M27,5h1M28,5h1M30,5h1M36,5h1M0,6h1M1,6h1M2,6h1M3,6h1M4,6h1M5,6h1M6,6h1M8,6h1M10,6h1M12,6h1M14,6h1M16,6h1M18,6h1M20,6h1M22,6h1M24,6h1M26,6h1M28,6h1M30,6h1M31,6h1M32,6h1M33,6h1M34,6h1M35,6h1M36,6h1M8,7h1M12,7h1M14,7h1M15,7h1M18,7h1M19,7h1M24,7h1M27,7h1M2,8h1M3,8h1M4,8h1M6,8h1M8,8h1M9,8h1M11,8h1M13,8h1M14,8h1M16,8h1M17,8h1M18,8h1M19,8h1M21,8h1M22,8h1M23,8h1M24,8h1M25,8h1M28,8h1M29,8h1M30,8h1M31,8h1M34,8h1M35,8h1M36,8h1M2,9h1M3,9h1M4,9h1M9,9h1M10,9h1M11,9h1M12,9h1M13,9h1M14,9h1M15,9h1M16,9h1M17,9h1M21,9h1M22,9h1M26,9h1M28,9h1M31,9h1M33,9h1M35,9h1M0,10h1M2,10h1M3,10h1M4,10h1M5,10h1M6,10h1M7,10h1M12,10h1M14,10h1M15,10h1M17,10h1M19,10h1M20,10h1M27,10h1M28,10h1M30,10h1M32,10h1M34,10h1M36,10h1M1,11h1M2,11h1M5,11h1M8,11h1M9,11h1M11,11h1M12,11h1M15,11h1M19,11h1M24,11h1M27,11h1M28,11h1M29,11h1M30,11h1M31,11h1M32,11h1M35,11h1M1,12h1M2,12h1M3,12h1M6,12h1M9,12h1M11,12h1M16,12h1M18,12h1M20,12h1M24,12h1M25,12h1M26,12h1M27,12h1M30,12h1M31,12h1M32,12h1M34,12h1M36,12h1M0,13h1M4,13h1M7,13h1M8,13h1M9,13h1M11,13h1M12,13h1M13,13h1M15,13h1M16,13h1M17,13h1M18,13h1M20,13h1M26,13h1M28,13h1M29,13h1M33,13h1M0,14h1M2,14h1M3,14h1M6,14h1M9,14h1M10,14h1M12,14h1M14,14h1M15,14h1M16,14h1M18,14h1M20,14h1M27,14h1M30,14h1M31,14h1M32,14h1M33,14h1M34,14h1M35,14h1M36,14h1M1,15h1M2,15h1M4,15h1M5,15h1M7,15h1M8,15h1M17,15h1M18,15h1M20,15h1M21,15h1M22,15h1M23,15h1M24,15h1M26,15h1M27,15h1M31,15h1M36,15h1M1,16h1M2,16h1M3,16h1M6,16h1M10,16h1M11,16h1M12,16h1M14,16h1M15,16h1M17,16h1M20,16h1M21,16h1M23,16h1M25,16h1M29,16h1M30,16h1M31,16h1M32,16h1M33,16h1M34,16h1M0,17h1M1,17h1M2,17h1M4,17h1M7,17h1M8,17h1M9,17h1M10,17h1M11,17h1M13,17h1M15,17h1M18,17h1M22,17h1M24,17h1M26,17h1M28,17h1M29,17h1M33,17h1M34,17h1M3,18h1M5,18h1M6,18h1M8,18h1M9,18h1M10,18h1M12,18h1M14,18h1M15,18h1M17,18h1M18,18h1M20,18h1M21,18h1M24,18h1M25,18h1M27,18h1M29,18h1M30,18h1M31,18h1M33,18h1M34,18h1M35,18h1M36,18h1M2,19h1M3,19h1M5,19h1M7,19h1M12,19h1M14,19h1M15,19h1M17,19h1M22,19h1M26,19h1M28,19h1M30,19h1M33,19h1M0,20h1M3,20h1M4,20h1M5,20h1M6,20h1M8,20h1M11,20h1M12,20h1M13,20h1M16,20h1M17,20h1M18,20h1M19,20h1M20,20h1M21,20h1M25,20h1M26,20h1M27,20h1M28,20h1M29,20h1M30,20h1M31,20h1M32,20h1M34,20h1M0,21h1M1,21h1M3,21h1M5,21h1M7,21h1M8,21h1M11,21h1M14,21h1M15,21h1M17,21h1M21,21h1M26,21h1M29,21h1M31,21h1M33,21h1M34,21h1M0,22h1M2,22h1M6,22h1M8,22h1M12,22h1M17,22h1M18,22h1M19,22h1M23,22h1M25,22h1M27,22h1M31,22h1M33,22h1M35,22h1M36,22h1M1,23h1M2,23h1M3,23h1M5,23h1M7,23h1M9,23h1M10,23h1M11,23h1M14,23h1M15,23h1M18,23h1M20,23h1M21,23h1M23,23h1M33,23h1M35,23h1M1,24h1M2,24h1M3,24h1M6,24h1M10,24h1M11,24h1M12,24h1M16,24h1M18,24h1M20,24h1M21,24h1M27,24h1M28,24h1M29,24h1M30,24h1M32,24h1M34,24h1M36,24h1M0,25h1M1,25h1M3,25h1M5,25h1M8,25h1M9,25h1M10,25h1M12,25h1M15,25h1M19,25h1M20,25h1M24,25h1M25,25h1M29,25h1M31,25h1M33,25h1M34,25h1M35,25h1M0,26h1M2,26h1M3,26h1M4,26h1M5,26h1M6,26h1M7,26h1M10,26h1M12,26h1M14,26h1M15,26h1M16,26h1M17,26h1M18,26h1M19,26h1M25,26h1M27,26h1M31,26h1M32,26h1M33,26h1M35,26h1M36,26h1M0,27h1M3,27h1M5,27h1M8,27h1M15,27h1M16,27h1M17,27h1M18,27h1M21,27h1M22,27h1M24,27h1M27,27h1M30,27h1M31,27h1M36,27h1M0,28h1M2,28h1M3,28h1M6,28h1M7,28h1M9,28h1M10,28h1M12,28h1M13,28h1M15,28h1M21,28h1M22,28h1M24,28h1M25,28h1M26,28h1M28,28h1M29,28h1M30,28h1M31,28h1M32,28h1M33,28h1M34,28h1M35,28h1M36,28h1M8,29h1M13,29h1M14,29h1M15,29h1M16,29h1M17,29h1M18,29h1M20,29h1M22,29h1M23,29h1M28,29h1M32,29h1M33,29h1M35,29h1M0,30h1M1,30h1M2,30h1M3,30h1M4,30h1M5,30h1M6,30h1M9,30h1M10,30h1M11,30h1M12,30h1M13,30h1M15,30h1M18,30h1M20,30h1M22,30h1M24,30h1M26,30h1M28,30h1M30,30h1M32,30h1M34,30h1M36,30h1M0,31h1M6,31h1M9,31h1M10,31h1M11,31h1M13,31h1M14,31h1M20,31h1M22,31h1M24,31h1M28,31h1M32,31h1M33,31h1M35,31h1M0,32h1M2,32h1M3,32h1M4,32h1M6,32h1M8,32h1M10,32h1M11,32h1M14,32h1M15,32h1M16,32h1M17,32h1M24,32h1M25,32h1M26,32h1M27,32h1M28,32h1M29,32h1M30,32h1M31,32h1M32,32h1M34,32h1M36,32h1M0,33h1M2,33h1M3,33h1M4,33h1M6,33h1M8,33h1M10,33h1M11,33h1M13,33h1M18,33h1M19,33h1M20,33h1M27,33h1M29,33h1M30,33h1M31,33h1M32,33h1M33,33h1M34,33h1M0,34h1M2,34h1M3,34h1M4,34h1M6,34h1M8,34h1M9,34h1M17,34h1M19,34h1M22,34h1M23,34h1M24,34h1M25,34h1M27,34h1M29,34h1M33,34h1M36,34h1M0,35h1M6,35h1M9,35h1M13,35h1M14,35h1M15,35h1M16,35h1M17,35h1M18,35h1M21,35h1M22,35h1M23,35h1M28,35h1M30,35h1M31,35h1M32,35h1M36,35h1M0,36h1M1,36h1M2,36h1M3,36h1M4,36h1M5,36h1M6,36h1M9,36h1M11,36h1M13,36h1M14,36h1M16,36h1M19,36h1M20,36h1M23,36h1M25,36h1M26,36h1M33,36h1M34,36h1M35,36h1M36,36h1" />
            </g>
        </svg>
    }
 }
