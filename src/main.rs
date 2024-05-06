#![windows_subsystem = "windows"]

mod utils;

use std::{env::current_exe, mem::size_of, path::Path, ptr::null_mut};

use native_windows_derive::NwgUi;
use native_windows_gui::{
    dispatch_thread_events, init, modal_fatal_message, modal_info_message, modal_message,
    simple_message, Button, FileDialog, Font, GridLayout, Icon, Label, MessageButtons,
    MessageChoice, MessageIcons, MessageParams, NativeUi, TextInput, Window,
};
use regex::Regex;
use utils::AsLPSZ;
use winapi::um::{
    commctrl::BCM_SETSHIELD,
    handleapi::CloseHandle,
    shellapi::{
        SHFileOperationW, ShellExecuteExW, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_NOCONFIRMMKDIR,
        FO_COPY, FO_DELETE, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, SHFILEOPSTRUCTW,
    },
    synchapi::WaitForSingleObject,
    winbase::INFINITE,
    winuser::{SendMessageW, SW_SHOW},
};

const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_resource(source_bin: Some(ICON_BYTES))]
    icon: Icon,

    #[nwg_control(size: (500, 130), title: "链接移动工具", flags: "WINDOW|VISIBLE", icon: Some(&data.icon))]
    #[nwg_events( OnWindowClose: [stop_thread_dispatch()] )]
    window: Window,

    #[nwg_resource(title: "Open File", action: FileDialogAction::OpenDirectory)]
    dir_dialog: FileDialog,

    #[nwg_layout(parent: window)]
    layout: GridLayout,

    #[nwg_control(text: "源目录")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    src_label: Label,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, row: 0, col: 1, col_span: 4)]
    src_input: TextInput,

    #[nwg_control(text: "选择")]
    #[nwg_layout_item(layout: layout, row: 0, col: 5)]
    #[nwg_events( OnButtonClick: [App::src_select] )]
    src_button: Button,

    #[nwg_control(text: "目标目录")]
    #[nwg_layout_item(layout: layout, row: 1, col: 0)]
    dst_label: Label,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, row: 1, col: 1, col_span: 4)]
    dst_input: TextInput,

    #[nwg_control(text: "选择")]
    #[nwg_layout_item(layout: layout, row: 1, col: 5)]
    #[nwg_events( OnButtonClick: [App::dst_select] )]
    dst_button: Button,

    #[nwg_control(text: "开始链接移动")]
    #[nwg_layout_item(layout: layout, row: 2, col: 2, col_span: 2)]
    #[nwg_events( OnButtonClick: [App::run_confirm] )]
    run_button: Button,
}

impl App {
    fn src_select(&self) {
        if self.dir_dialog.run(Some(&self.window)) {
            if let Ok(dir) = self.dir_dialog.get_selected_item() {
                let s = dir.to_str().unwrap();
                self.src_input.set_text(&s);
                self.dst_input.set_text(&format!("D{}", s[1..].to_string()))
            }
        }
    }

    fn dst_select(&self) {
        if self.dir_dialog.run(Some(&self.window)) {
            if let Ok(dir) = self.dir_dialog.get_selected_item() {
                self.dst_input.set_text(&dir.into_string().unwrap())
            }
        }
    }

    fn run_cli(&self, src: String, dst: String) {
        unsafe {
            let v_verb: Vec<u16> = "runas".as_lpsz();
            let v_file: Vec<u16> = format!(
                "{}",
                current_exe()
                    .expect("Failed to current_exe")
                    .to_str()
                    .expect("Failed to to_str"),
            )
            .as_lpsz();
            let v_param: Vec<u16> = format!("\"{src}\" \"{dst}\"").as_lpsz();

            let mut info = SHELLEXECUTEINFOW {
                cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
                fMask: SEE_MASK_NOCLOSEPROCESS,
                hwnd: null_mut(),
                lpVerb: v_verb.as_ptr(),
                lpFile: v_file.as_ptr(),
                lpParameters: v_param.as_ptr(),
                lpDirectory: null_mut(),
                nShow: SW_SHOW,
                hInstApp: null_mut(),
                lpIDList: null_mut(),
                lpClass: null_mut(),
                hkeyClass: null_mut(),
                dwHotKey: 0,
                hMonitor: null_mut(),
                hProcess: null_mut(),
            };
            if ShellExecuteExW(&mut info) == 1 {
                WaitForSingleObject(info.hProcess, INFINITE);
                CloseHandle(info.hProcess);
            } else {
                modal_fatal_message(&self.window, "错误", "提权失败");
            }
        };
    }

    fn run_confirm(&self) {
        let rules: Vec<(&str, Option<Regex>, Option<Regex>, &str)> = vec![
            reg_rule!("prevent", r"^[A-Z]:?\\?\s*$", None, "禁止链接整个磁盘"),
            reg_rule!("prevent", None, r"^[A-Z]:?\\?\s*$", "禁止链接整个磁盘"),
            reg_rule!(
                "prevent",
                r"^C:\\Program Files( \(x86\))?\\?\s*$",
                None,
                "链接整个\"Program Files\"将会导致所有UWP应用不可用"
            ),
        ];

        let src = self.src_input.text();
        let dst = self.dst_input.text();
        let src_final = src.clone();
        let mut dst_final = dst.clone();
        if src.is_empty() {
            modal_info_message(&self.window, "提示", "请选择源目录");
            return;
        }
        if src.is_empty() {
            modal_info_message(&self.window, "提示", "请选择目标目录");
            return;
        }
        if src == dst {
            modal_info_message(&self.window, "提示", "源目录不能和目标目录相同");
            return;
        }

        let src_path = Path::new(&src);
        let dst_path = Path::new(&dst);

        for rule in rules {
            if rule.0 == "prevent" {
                if let Some(r) = &rule.1 {
                    if r.captures(src_path.to_str().unwrap()).is_some() {
                        modal_info_message(&self.window, "提示", rule.3);
                        return;
                    }
                }
                if let Some(r) = &rule.1 {
                    if r.captures(dst_path.to_str().unwrap()).is_some() {
                        modal_info_message(&self.window, "提示", rule.3);
                        return;
                    }
                }
            }
        }

        if src_path.exists() {
            if dst_path.exists() {
                if modal_message(
                    &self.window,
                    &MessageParams {
                        title: "提示",
                        content: "目标目录存在，确认覆盖目标目录吗？",
                        buttons: MessageButtons::OkCancel,
                        icons: MessageIcons::Info,
                    },
                ) != MessageChoice::Ok
                {
                    return;
                }
                dst_final = dst_path
                    .parent()
                    .expect("Failed to get parent")
                    .to_str()
                    .expect("Failed to to_str")
                    .to_string();
            }
            if modal_message(
                &self.window,
                &MessageParams {
                    title: "提示",
                    content: format!("确认将\"{src}\"链接移动到\"{dst}\"？").as_str(),
                    buttons: MessageButtons::OkCancel,
                    icons: MessageIcons::Info,
                },
            ) == MessageChoice::Ok
            {
                self.run_cli(src_final, dst_final);
            }
        } else {
            if dst_path.exists() {
                if modal_message(
                    &self.window,
                    &MessageParams {
                        title: "提示",
                        content: format!("确认将\"{dst}\"链接到\"{src}\"？").as_str(),
                        buttons: MessageButtons::OkCancel,
                        icons: MessageIcons::Info,
                    },
                ) == MessageChoice::Ok
                {
                    self.run_cli(src_final, dst_final);
                }
            } else {
                modal_info_message(&self.window, "提示", "源目录不存在，请选择正确的路径");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let src = args[1].clone();
        let dst = args[2].clone();
        let src_path = Path::new(&src);
        if src_path.exists() {
            let v_from: Vec<u16> = src.as_lpsz();
            let v_to: Vec<u16> = dst.as_lpsz();

            let r = unsafe {
                SHFileOperationW(&mut SHFILEOPSTRUCTW {
                    hwnd: null_mut(),
                    wFunc: FO_COPY as u32,
                    pFrom: v_from.as_ptr(),
                    pTo: v_to.as_ptr(),
                    fFlags: FOF_ALLOWUNDO | FOF_NOCONFIRMMKDIR | FOF_NOCONFIRMATION,
                    fAnyOperationsAborted: 0,
                    hNameMappings: null_mut(),
                    lpszProgressTitle: null_mut(),
                })
            };
            if r != 0 {
                simple_message("错误", format!("目录拷贝失败（0x{r:02X}）").as_str());
                return;
            }

            let r = unsafe {
                SHFileOperationW(&mut SHFILEOPSTRUCTW {
                    hwnd: null_mut(),
                    wFunc: FO_DELETE as u32,
                    pFrom: v_from.as_ptr(),
                    pTo: null_mut(),
                    fFlags: FOF_ALLOWUNDO | FOF_NOCONFIRMATION,
                    fAnyOperationsAborted: 0,
                    hNameMappings: null_mut(),
                    lpszProgressTitle: null_mut(),
                })
            };
            if r != 0 {
                simple_message("错误", format!("目录删除失败（0x{r:02X}）").as_str());
                return;
            }
            simple_message("提示", "链接成功");
        }

        let v_verb: Vec<u16> = "runas".as_lpsz();
        let v_file: Vec<u16> = "cmd.exe".as_lpsz();
        let v_param: Vec<u16> = format!("/c mklink /J \"{src}\" \"{dst}\"").as_lpsz();
        let mut info = SHELLEXECUTEINFOW {
            cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOCLOSEPROCESS,
            hwnd: null_mut(),
            lpVerb: v_verb.as_ptr(),
            lpFile: v_file.as_ptr(),
            lpParameters: v_param.as_ptr(),
            lpDirectory: null_mut(),
            nShow: SW_SHOW,
            hInstApp: null_mut(),
            lpIDList: null_mut(),
            lpClass: null_mut(),
            hkeyClass: null_mut(),
            dwHotKey: 0,
            hMonitor: null_mut(),
            hProcess: null_mut(),
        };
        if unsafe { ShellExecuteExW(&mut info) } == 1 {
            unsafe { WaitForSingleObject(info.hProcess, INFINITE) };
            unsafe { CloseHandle(info.hProcess) };
        } else {
            simple_message("错误", "链接创建失败");
        }
        return;
    }

    init().expect("Failed to init Native Windows GUI");
    let mut font = Font::default();
    Font::builder()
        .family("Segoe UI")
        .size_absolute(11)
        .build(&mut font)
        .expect("Failed to set default font");
    Font::set_global_default(Some(font));
    let app = App::build_ui(Default::default()).expect("Failed to build UI");
    unsafe {
        SendMessageW(
            app.run_button.handle.hwnd().unwrap(),
            BCM_SETSHIELD,
            0,
            0xFFFFFFFF,
        )
    };
    dispatch_thread_events();
}
