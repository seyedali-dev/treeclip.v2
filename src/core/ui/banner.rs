use colored::Colorize;
use rand::Rng;

const BANNERS: &[&str] = &[
    r#"
    ╔═══════════════════════════════════════════════════╗
    ║   🌳  T R E E C L I P  🌳                         ║
    ║     Traverse & Extract with Style!                ║
    ║                                                   ║
    ║     (づ｡◕‿‿◕｡)づ  Let's gather some leaves!     ║
    ╚═══════════════════════════════════════════════════╝
    "#,
    r#"
    ╭─────────────────────────────────────────────────╮
    │   ✨  T R E E C L I P  ✨                       │
    │    Your friendly code extraction companion!     │
    │                                                 │
    │    ♡( ◡‿◡ )  Ready to explore your files~      │
    ╰─────────────────────────────────────────────────╯
    "#,
    r#"
    ┌───────────────────────────────────────────────────┐
    │   🎄  T R E E C L I P  🎄                         │
    │      Fast • Simple • Cute                         │
    │                                                   │
    │   ヾ(⌐■_■)ノ♪  Time to clip that tree!          │
    └───────────────────────────────────────────────────┘
    "#,
];

pub fn print_welcome() {
    let mut rng = rand::rng();
    let banner = BANNERS[rng.random_range(0..BANNERS.len())];
    println!("{}", banner.bright_magenta());
}

const GOODBYE_MESSAGES: &[&str] = &[
    "✨ Mission accomplished! Time to shine!",
    "🎯 All done! Maybe grab a cookie? 🍪",
    "🌟 Great work! Your code is ready for takeoff!",
    "💫 TreeClip adventure complete! See you next time~",
    "🎉 Perfect! Everything extracted successfully!",
    "✅ Nailed it! Your files are all bundled up!",
    "🚀 Launch ready! Your code awaits!",
    "🎊 Fantastic! Another tree successfully clipped!",
];

pub fn print_goodbye() {
    println!("\n{}", "━".repeat(55).bright_cyan());

    let mut rng = rand::rng();
    let message = GOODBYE_MESSAGES[rng.random_range(0..GOODBYE_MESSAGES.len())];

    println!("    {}", message.bright_green().bold());
    println!(
        "    {} {}",
        get_random_kaomoji(),
        "Have a wonderful day!".bright_yellow()
    );
    println!("{}\n", "━".repeat(55).bright_cyan());
}

const KAOMOJIS: &[&str] = &[
    "ʕ•ᴥ•ʔ",
    "(◕‿◕✿)",
    "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧",
    "✧･ﾟ: *✧･ﾟ:*",
    "(づ｡◕‿‿◕｡)づ",
    "(っ◕‿◕)っ",
    "♡( ◡‿◡ )",
    "(●´ω｀●)",
    "٩(◕‿◕｡)۶",
    "ヽ(•‿•)ノ",
    "(ﾉ´ з `)ノ",
    "(´｡• ω •｡`)",
    "☆ﾟ･*:.｡.☆(￣ω￣)/",
    "(๑˃ᴗ˂)ﻭ",
    "╰( ´・ω・)つ──☆",
    "ヾ(⌐■_■)ノ♪",
    "ヾ(☆▽☆)",
    "(ﾉ>ω<)ﾉ",
    "(◠‿◠✿)",
    "(ﾉ^ヮ^)ﾉ*:・ﾟ✧",
];

pub fn get_random_kaomoji() -> &'static str {
    let mut rng = rand::rng();
    KAOMOJIS[rng.random_range(0..KAOMOJIS.len())]
}
