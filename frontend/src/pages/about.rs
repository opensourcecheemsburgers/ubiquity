use gloo::utils::{document, window};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::{contexts::config::use_config, components::{header::Header, link_btn::LinkBtn, drawer::Drawer}, icons::{MoneroQr, BitcoinQr}};
use crate::pages::background::Background;
use web_sys::{HtmlDialogElement, Navigator};

#[function_component(About)]
pub fn about() -> Html {
    let mobile_ui = use_config().is_mobile_ui();

    let source_code_link = "https://github.com/opensourcecheemsburgers/Ubiquity";
    let tauri_link = "https://tauri.app";
    let yew_link = "https://yew.rs";
    let daisyui_link = "https://daisyui.com";
    let tailwind_link = "https://tailwindcss.com";

    let mut hero_classes = classes!(        
        "hero",
        "self-center",
        "w-full",
    );

    match mobile_ui {
        true => hero_classes.push("h-[calc(100svh)]"),
        false => hero_classes.push("h-[calc(100vh-4rem)]")
    }

    let hero_content_classes = classes!("hero-content");

    let article_classes = classes!(
        "prose-xl",
        "prose-a:text-primary",
        "hover:prose-a:text-primary-focus",
        "font-mono",
        "cursor-default"
    );

    let header_classes = classes!("font-display");

    let text_classes = classes!(        
        "lg:text-2xl",
        "text-xl"
    );

    let links_container_classes = classes!(
        "not-prose",
        "flex",
        "flex-col",
        "lg:flex-row",
        "items-start",
        "justify-start",
        "gap-5"
    );

    html! {
        <Drawer>
            <Background>
                <Header />
                <div class={hero_classes}>
                    <div class={hero_content_classes}>
                        <article class={article_classes}>
                            <h1 class={header_classes.clone()}>{"Ubiquity"}</h1>
                            <p class={text_classes.clone()}>
                                {"An open-source, cross-platform markdown editor made with Tauri."}
                            </p>
                            <p class={text_classes.clone()}>
                                {"Ubiquity is written in Rust and its frontend is built with Yew and DaisyUI."}
                            </p>
                            <h2 class="font-display">{"Donate"}</h2>
                            <p class={text_classes.clone()}>{"Ubiquity is a FOSS project. Support my work and further
                                development."}
                            </p>
                            <p class={text_classes.clone()}>{"All donations are greatly appreciated, no matter the
                                size."}
                            </p>
                            <div class={links_container_classes.clone()}>
                                <PaypalDonationBtn />
                                <MoneroDonationBtn />
                                <BitcoinDonationBtn />
                            </div>
                            <h2 class="font-display">{"Links"}</h2>
                            <div class={links_container_classes}>
                                <LinkBtn title={"Source Code"} link={source_code_link} />
                                <LinkBtn title={"Tauri"} link={tauri_link} />
                                <LinkBtn title={"Yew"} link={yew_link} />
                                <LinkBtn title={"Tailwind"} link={tailwind_link} />
                                <LinkBtn title={"Daisy UI"} link={daisyui_link} />
                            </div>
                        </article>
                    </div>
                </div>
            </Background>
        </Drawer>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CopyAddressBtnProps {
    pub address: String,
}

#[function_component(CopyAddressBtn)]
pub fn copy_address_btn(props: &CopyAddressBtnProps) -> Html {
    let address = props.address.clone();
    let copy = Callback::from(move |_| {
        tauri_sys::clipboard::write_text(&address);
    });

    html! {
        <button onclick={copy}>{"Copy"}</button>
    }
}

#[function_component(PaypalDonationBtn)]
pub fn paypal_donation_btn() -> Html {
    html! {
        <div data-tip={"Paypal Donations"} class="tooltip tooltip-bottom tooltip-info">
            <a class="btn btn-lg btn-outline" href="https://www.paypal.com/donate/?hosted_button_id=A8UNN2A4D3P3G">
                <svg class="mr-4 h-8 w-8" viewBox="0 0 256 302" version="1.1"
                    xmlns="http://www.w3.org/2000/svg">
                    <g>
                        <path
                            d="M217.168476,23.5070146 C203.234077,7.62479651 178.045612,0.815753338 145.823355,0.815753338 L52.3030619,0.815753338 C45.7104431,0.815753338 40.1083819,5.6103852 39.0762042,12.1114399 L0.136468302,259.076601 C-0.637664968,263.946149 3.13311322,268.357876 8.06925331,268.357876 L65.804612,268.357876 L80.3050438,176.385849 L79.8555471,179.265958 C80.8877248,172.764903 86.4481659,167.970272 93.0324607,167.970272 L120.46841,167.970272 C174.366398,167.970272 216.569147,146.078116 228.897012,82.7490197 C229.263268,80.8761167 229.579581,79.0531577 229.854273,77.2718188 C228.297683,76.4477414 228.297683,76.4477414 229.854273,77.2718188 C233.525163,53.8646924 229.829301,37.9325302 217.168476,23.5070146"
                            fill="#27346A"></path>
                        <path
                            d="M102.396976,68.8395929 C103.936919,68.1070797 105.651665,67.699203 107.449652,67.699203 L180.767565,67.699203 C189.449511,67.699203 197.548776,68.265236 204.948824,69.4555699 C207.071448,69.7968545 209.127479,70.1880831 211.125242,70.6375799 C213.123006,71.0787526 215.062501,71.5781934 216.943728,72.1275783 C217.884341,72.4022708 218.808307,72.6852872 219.715624,72.9849517 C223.353218,74.2002577 226.741092,75.61534 229.854273,77.2718188 C233.525163,53.8563683 229.829301,37.9325302 217.168476,23.5070146 C203.225753,7.62479651 178.045612,0.815753338 145.823355,0.815753338 L52.2947379,0.815753338 C45.7104431,0.815753338 40.1083819,5.6103852 39.0762042,12.1114399 L0.136468302,259.068277 C-0.637664968,263.946149 3.13311322,268.349552 8.0609293,268.349552 L65.804612,268.349552 L95.8875974,77.5798073 C96.5035744,73.6675208 99.0174265,70.4627756 102.396976,68.8395929 Z"
                            fill="#27346A"></path>
                        <path
                            d="M228.897012,82.7490197 C216.569147,146.069792 174.366398,167.970272 120.46841,167.970272 L93.0241367,167.970272 C86.4398419,167.970272 80.8794007,172.764903 79.8555471,179.265958 L61.8174095,293.621258 C61.1431644,297.883153 64.4394738,301.745495 68.7513129,301.745495 L117.421821,301.745495 C123.182038,301.745495 128.084882,297.550192 128.983876,291.864891 L129.458344,289.384335 L138.631407,231.249423 L139.222412,228.036354 C140.121406,222.351053 145.02425,218.15575 150.784467,218.15575 L158.067979,218.15575 C205.215193,218.15575 242.132193,199.002194 252.920115,143.605884 C257.423406,120.456802 255.092683,101.128442 243.181019,87.5519756 C239.568397,83.4399129 235.081754,80.0437153 229.854273,77.2718188 C229.571257,79.0614817 229.263268,80.8761167 228.897012,82.7490197 L228.897012,82.7490197 Z"
                            fill="#2790C3"></path>
                        <path
                            d="M216.952052,72.1275783 C215.070825,71.5781934 213.13133,71.0787526 211.133566,70.6375799 C209.135803,70.1964071 207.071448,69.8051785 204.957148,69.4638939 C197.548776,68.265236 189.457835,67.699203 180.767565,67.699203 L107.457976,67.699203 C105.651665,67.699203 103.936919,68.1070797 102.4053,68.8479169 C99.0174265,70.4710996 96.5118984,73.6675208 95.8959214,77.5881313 L80.3133678,176.385849 L79.8638711,179.265958 C80.8877248,172.764903 86.4481659,167.970272 93.0324607,167.970272 L120.476734,167.970272 C174.374722,167.970272 216.577471,146.078116 228.905336,82.7490197 C229.271592,80.8761167 229.579581,79.0614817 229.862597,77.2718188 C226.741092,75.623664 223.361542,74.2002577 219.723948,72.9932757 C218.816631,72.6936112 217.892665,72.4022708 216.952052,72.1275783"
                            fill="#1F264F"></path>
                    </g>
                </svg>
                {"Paypal"}
            </a>
        </div>
    }
}

#[function_component(MoneroDonationBtn)]
pub fn monero_donation_btn() -> Html { 
    html! {
        <>
            <div data-tip={"Monero Donation Address"} class="tooltip tooltip-bottom tooltip-info">
                <label for="xmr-modal" class="btn btn-lg btn-outline">
                <svg class="mr-4 h-8 w-8" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid">
                    <path d="M127.998 0C57.318 0 0 57.317 0 127.999c0 14.127 2.29 27.716 6.518 40.43H44.8V60.733l83.2 83.2 83.198-83.2v107.695h38.282c4.231-12.714 6.521-26.303 6.521-40.43C256 57.314 198.681 0 127.998 0" fill="#F60"/><path d="M108.867 163.062l-36.31-36.311v67.765H18.623c22.47 36.863 63.051 61.48 109.373 61.48s86.907-24.617 109.374-61.48h-53.933V126.75l-36.31 36.31-19.13 19.129-19.128-19.128h-.002z" fill="#4C4C4C"/>
                </svg>
                {"Monero"}
                </label>
            </div>

            <input type="checkbox" id="xmr-modal" class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"XMR Donation"}</h3>
                    <p class="py-4">{"Donate by scanning the QR code or copying the address below."}</p>
                    <MoneroQr />
                    <p class="pt-4">{"XMR Address:"}</p>
                    <p class="break-all">
                        {"86ywTu1eDABJB2CxGf9mwMbX6ibfc7pga1XusQCrfvMYdNpNPto1BaDSbWeas13Nkg3iLigkzS7wSGc8YoQxEkHhHdrA4Ni"}
                    </p>
                    <div class="modal-action">
                        <label for="xmr-modal" class="btn">{"Ok"}</label>
                    </div>
                </div>
            </div>
        </>
    }
}

#[function_component(BitcoinDonationBtn)]
pub fn bitcoin_donation_btn() -> Html {  
    html! {
        <>
            <div data-tip={"Bitcoin Donation Address"} class="tooltip tooltip-bottom tooltip-info">
                <label for="btc-modal" class="btn btn-lg btn-outline">
                    <svg class="mr-4 h-8 w-8" xmlns="http://www.w3.org/2000/svg" version="1.1"
                        viewBox="0 0 4091.27 4091.73">
                        <g id="Layer_x0020_1">
                            <metadata id="CorelCorpID_0Corel-Layer" />
                            <g id="_1421344023328">
                                <path fill="#F7931A" fill-rule="nonzero"
                                    d="M4030.06 2540.77c-273.24,1096.01 -1383.32,1763.02 -2479.46,1489.71 -1095.68,-273.24 -1762.69,-1383.39 -1489.33,-2479.31 273.12,-1096.13 1383.2,-1763.19 2479,-1489.95 1096.06,273.24 1763.03,1383.51 1489.76,2479.57l0.02 -0.02z" />
                                <path fill="white" fill-rule="nonzero"
                                    d="M2947.77 1754.38c40.72,-272.26 -166.56,-418.61 -450,-516.24l91.95 -368.8 -224.5 -55.94 -89.51 359.09c-59.02,-14.72 -119.63,-28.59 -179.87,-42.34l90.16 -361.46 -224.36 -55.94 -92 368.68c-48.84,-11.12 -96.81,-22.11 -143.35,-33.69l0.26 -1.16 -309.59 -77.31 -59.72 239.78c0,0 166.56,38.18 163.05,40.53 90.91,22.69 107.35,82.87 104.62,130.57l-104.74 420.15c6.26,1.59 14.38,3.89 23.34,7.49 -7.49,-1.86 -15.46,-3.89 -23.73,-5.87l-146.81 588.57c-11.11,27.62 -39.31,69.07 -102.87,53.33 2.25,3.26 -163.17,-40.72 -163.17,-40.72l-111.46 256.98 292.15 72.83c54.35,13.63 107.61,27.89 160.06,41.3l-92.9 373.03 224.24 55.94 92 -369.07c61.26,16.63 120.71,31.97 178.91,46.43l-91.69 367.33 224.51 55.94 92.89 -372.33c382.82,72.45 670.67,43.24 791.83,-303.02 97.63,-278.78 -4.86,-439.58 -206.26,-544.44 146.69,-33.83 257.18,-130.31 286.64,-329.61l-0.07 -0.05zm-512.93 719.26c-69.38,278.78 -538.76,128.08 -690.94,90.29l123.28 -494.2c152.17,37.99 640.17,113.17 567.67,403.91zm69.43 -723.3c-63.29,253.58 -453.96,124.75 -580.69,93.16l111.77 -448.21c126.73,31.59 534.85,90.55 468.94,355.05l-0.02 0z" />
                            </g>
                        </g>
                    </svg>
                    {"Bitcoin"}
                </label>
            </div>

            <input type="checkbox" id="btc-modal" class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"BTC Donation"}</h3>
                    <p class="py-4">{"Donate by scanning the QR code or copying the address below."}</p>
                    <BitcoinQr />
                    <p class="pt-4">{"BTC Address:"}</p>
                    <p class="break-all">
                        {"bc1q8r90zc8j8a2rvkq4ds8374pxh3rpccxgnjx5x2"}
                    </p>
                    <div class="modal-action">
                        <label for="btc-modal" class="btn">{"Ok"}</label>
                    </div>
                </div>
            </div>
        </>
    }
}
