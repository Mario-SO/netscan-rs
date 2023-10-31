use console::{style, Emoji, Style, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use std::net::Ipv4Addr;

pub fn init() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    // Header
    println!("{}", style("scan_rs by mariodev").bold().underlined());
    println!();
}

pub fn select_interface() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the interface name")
        .interact()
        .unwrap()
}

pub fn print_error(message: &str) {
    let bold = Style::new().bold();
    let red = Style::new().red().on_black();
    let cross_mark = Emoji("❌ ", "");

    eprintln!(
        "{}{}{}",
        bold.apply_to("Error: "),
        red.apply_to(cross_mark),
        message
    );
}

pub fn print_scanning_message(interface_name: &str) {
    let bold = Style::new().bold();
    let blue = Style::new().blue();
    let info_mark = Emoji("💡 ", "");

    println!();
    println!(
        "{}Scanning on interface: {}",
        bold.apply_to(blue.apply_to(info_mark)),
        interface_name
    );
    println!();
}

pub fn create_progress_bar(total_ips: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_ips);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {wide_bar:.green/red} {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}

pub fn print_device_found(ip: Ipv4Addr, mac: &str) {
    let green = Style::new().green();
    let yellow = Style::new().yellow();
    let blue = Style::new().blue();
    let check_mark = Emoji("✅ ", "");

    println!(
        "{}Found device with IP: {}, MAC: {}",
        green.apply_to(check_mark),
        yellow.apply_to(ip),
        blue.apply_to(mac)
    );
}
