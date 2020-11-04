#[macro_use]
extern crate penrose;

use penrose::{
    draw::{dwm_bar, TextStyle, XCBDraw},
    helpers::index_selectors,
    core::bindings::KeyBindings,
    Backward, Config, Forward, Less, More, Result, WindowManager, XcbConnection,
};

use std::collections::HashMap;

const BAR_BG: u32 = 0x282a36ff;
//const EMPTY_WS: u32 = 0x6272A4FF;
const BAR_EMPTY_WS: u32 = 0x44475aff;
const BAR_FG: u32 = 0xff79c6ff;

fn key_bindings(config: &Config) -> KeyBindings {
    gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-q" => run_internal!(kill_client);
        "M-Tab" => run_internal!(toggle_workspace);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward);
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-i" => run_internal!(update_max_main, More);
        "M-d" => run_internal!(update_max_main, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-h" => run_internal!(update_main_ratio, Less);
        "M-S-e" => run_internal!(exit);
        "M-p" => run_external!("dmenu_run");
        "M-Return" => run_external!("alacritty");
        "M-S-x" => run_external!("slock");

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces.len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces.len()) ];
        };

    }
}

fn main() -> Result<()> {
    let mut config = Config::default();
    let key_bindings = key_bindings(&config);

    let workspaces = &["1", "2", "3", "4", "5", "6"];
    let style = TextStyle {
        font: "Ubuntu".to_string(),
        point_size: 11,
        fg: 0xf8f8f2ff.into(),
        bg: Some(BAR_BG.into()),
        padding: (2.0, 2.0),
    };

    let bar = dwm_bar(
        Box::new(XCBDraw::new()?),
        18,
        &style,
        0xbd93f9ff,
        0x44475aff,
        workspaces,
    )?;

    config.hooks.push(Box::new(bar));

    let conn = XcbConnection::new()?;
    let mut wm = WindowManager::init(config, &conn);
    wm.grab_keys_and_run(key_bindings, HashMap::new());

    Ok(())
}
