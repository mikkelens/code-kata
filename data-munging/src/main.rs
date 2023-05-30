use std::collections::HashSet;
use std::str::FromStr;
fn main() {
    part_1();
    part_2();
}

// --- PART 2 ---
#[allow(unused)]
fn part_2() {
    let input = include_str!("football.txt");
    let data_str = input.split_once('\n').expect("Could not split from top").1;

    let entry_iter = data_str.lines()
        .filter(|line| !line.contains("---") && !line.is_empty())
        .map(|line| line.parse::<TeamStats>().unwrap() );

    let for_against_difference_iter = entry_iter
        .map(|entry| u8::abs_diff(entry.F, entry.A));

    let min_f_a_difference = for_against_difference_iter.min().unwrap();
    println!("Min For/Against difference: {}", min_f_a_difference)
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
struct TeamStats {
    rank: u8,
    name: String,
    P: u8, // no idea, all "38"
    W: u8, // wins i presume
    L: u8, // losses i presume
    D: u8, // draws i presume
    F: u8, // goals *for*
    A: u8, // goals *against*
    Pts: u8, // points i pressume
}
impl FromStr for TeamStats {
    type Err = &'static str;

    #[allow(non_snake_case)]
    fn from_str(mut line: &str) -> Result<Self, Self::Err> {
        // trim left (assuming mandatory), seprate segment from line, then parse.
        line = line.trim_start();
        let (rank, mut line) = line.split_once(". ").ok_or("Couldn't split rank from line")?;
        let rank = rank.parse().map_err(|_| "Couldn't parse rank as u8")?;
        
        line = line.trim_start();
        let (name, mut line) = line.split_once(' ').ok_or("Couldn't split name from line")?;
        let name = name.to_string();

        line = line.trim_start();
        let (P, mut line) = line.split_once(' ').ok_or("Couldn't split P from line")?;
        let P = P.parse().map_err(|_| "Couldn't parse P as u8")?;
        
        line = line.trim_start();
        let (W, mut line) = line.split_once(' ').ok_or("Couldn't split W from line")?;
        let W = W.parse().map_err(|_| "Couldn't parse W as u8")?;
        
        line = line.trim_start();
        let (L, mut line) = line.split_once(' ').ok_or("Couldn't split L from line")?;
        let L = L.parse().map_err(|_| "Couldn't parse L as u8")?;
        
        line = line.trim_start();
        let (D, mut line) = line.split_once(' ').ok_or("Couldn't split D from line")?;
        let D = D.parse().map_err(|_| "Couldn't parse D as u8")?;
        
        line = line.trim_start();
        let (F, mut line) = line.split_once(' ').ok_or("Couldn't split F from line")?;
        let F = F.parse().map_err(|_| "Couldn't parse F as u8")?;
        
        line = line.trim_start_matches([' ', '-']); // remove whitespace *and* "-"
        let (A, mut line) = line.split_once(' ').ok_or("Couldn't split A from line")?;
        let A = A.parse().map_err(|_| "Couldn't parse A as u8")?;

        line = line.trim_start();
        let Pts = line.parse().map_err(|_| "Couldn't parse Pts as u8")?; // skip split at the end of line

        Ok(TeamStats {
            rank,
            name,
            P,
            W,
            L,
            D,
            F,
            A,
            Pts,
        })
    }
}

// --- PART 1 ---
#[allow(unused)]
fn part_1() {
    let input = include_str!("weather.txt");
    // dbg!(input);
    let data_str = input
        .split_once('\n').expect("split first time").1
        .split_once('\n').expect("split second time").1;
    
    let entry_iter = data_str.lines()
        .filter(|line| !line.contains("mo") && !line.is_empty())
        .map(|line| line.parse::<WeatherDataEntry>().unwrap());
    
    let temperature_differences: Vec<u8> = entry_iter
        .map(|entry| u8::abs_diff(*entry.MxT.val(), *entry.MnT.val()))
        .collect();
    
    let min_difference = temperature_differences.iter().min().expect("Could not find min value");
    println!("Min temperature difference: {}", min_difference);
    let max_difference = temperature_differences.iter().max().expect("Could not find max value");
    println!("Max temperature difference: {}", max_difference);
}

impl FromStr for WeatherDataEntry {
    type Err = &'static str;

    #[allow(non_snake_case)]
    fn from_str(mut line: &str) -> Result<Self, Self::Err> {
        // trim left (if mandatory field), seperate segment from total line, then parse.
        line = line.trim_start();
        // dbg!(line);
        let (Dy, mut line) = line.split_once(' ').ok_or("Couldn't split Dy from line")?;
        let Dy = Dy.parse().map_err(|_| "Couldn't parse Dy as u8")?;
        
        line = line.trim_start();
        // dbg!(line);
        let (MxT, mut line) = line.split_once(' ').ok_or("Couldn't split MxT from line")?;
        let MxT = MxT.parse().map_err(|_| "Couldn't parse MxT as u8 (*)")?;
 
        line = line.trim_start();
        // dbg!(line);
        let (MnT, mut line) = line.split_once(' ').ok_or("Couldn't split MnT from line")?;
        let MnT = MnT.parse().map_err(|_| "Couldn't parse MnT as u8 (*)")?;

        line = line.trim_start();
        // dbg!(line);
        let (AvT, mut line) = line.split_once(' ').ok_or("Couldn't split AvT from line")?;
        let AvT = AvT.parse().map_err(|_| "Couldn't parse AvT as f32")?;

        let HDDay = {
            // check if entry exists, then seperate and parse
            let (HDDay_entry, line_without_hdday_entry) = line.split_at(7);
            if HDDay_entry.chars().any(|c| c != ' ') {
                line = line_without_hdday_entry;
                let value = HDDay_entry.trim().parse().map_err(|_| "Couldn't parse HDDay entry as u8")?;
                Some(value)
            } else {
                None
            }
        };
        
        line = line.trim_start();
        // dbg!(line);
        let (AvDP, mut line) = line.split_once(' ').ok_or("Couldn't split AvDP from line")?;
        let AvDP = AvDP.parse().map_err(|_| "Couldn't parse AvDP as f32")?;
        
        const ONE_HR_P: () = (); // always empty
        line = line.trim_start();
        // dbg!(line);

        line = line.trim_start();
        // dbg!(line);
        let (TPcpn, mut line) = line.split_once(' ').ok_or("Couldn't split TPcpn from line")?;
        let TPcpn = TPcpn.parse().map_err(|_| "Couldn't parse TPcpn as f32")?;
        
        let WxType = {
            let mut WxType_mut = HashSet::new();
            let (WxType_entry, line_without_WxType_entry) = line.split_at(7);
            let WxType_chars = WxType_entry.chars().filter(|c| c != &' ');
            if WxType_chars.clone().count() > 0 {
                line = line_without_WxType_entry;
                for c in WxType_chars {
                    match c {
                        'R' => { WxType_mut.insert(WxType::R); },
                        'T' => { WxType_mut.insert(WxType::T); },
                        'F' => { WxType_mut.insert(WxType::F); },
                        'H' => { WxType_mut.insert(WxType::H); },
                        _ => panic!("Contained chars in WxType field that were not expected")
                    }
                }
            }
            WxType_mut
        };
        
        line = line.trim_start();
        // dbg!(line);
        let (PDir, mut line) = line.split_once(' ').ok_or("Couldn't split PDir from line")?;
        let PDir = PDir.parse().map_err(|_| "Couldn't parse PDir as u16")?;
        
        line = line.trim_start();
        // dbg!(line);
        let (AvSp, mut line) = line.split_once(' ').ok_or("Couldn't split AvSp from line")?;
        let AvSp = AvSp.parse().map_err(|_| "Couldn't parse AvSp as f32")?;
        
        line = line.trim_start();
        // dbg!(line);
        let (Dir, mut line) = line.split_once(' ').ok_or("Couldn't split Dir from line")?;
        let Dir = Dir.parse().map_err(|_| "Couldn't parse Dir as u16")?;
        
        line = line.trim_start();
        // dbg!(line);
        let (MxS, mut line) = line.split_once(' ').ok_or("Couldn't split MxS from line")?;
        let MxS = MxS.parse().map_err(|_| "Couldn't parse MxS as u8 (*)")?;

        line = line.trim_start();
        // dbg!(line);
        let (SkyC, mut line) = line.split_once(' ').ok_or("Couldn't split SkyC from line")?;
        let SkyC = SkyC.parse().map_err(|_| "Couldn't parse SkyC as f32")?;

        line = line.trim_start();
        // dbg!(line);
        let (MxR, mut line) = line.split_once(' ').ok_or("Couldn't split MxR from line")?;
        let MxR = MxR.parse().map_err(|_| "Couldn't parse MxR as u8")?;

        line = line.trim_start();
        // dbg!(line);
        let (MnR, mut line) = line.split_once(' ').ok_or("Couldn't split MnR from line")?;
        let MnR = MnR.parse().map_err(|_| "Couldn't parse MnR as u8")?;

        line = line.trim_start();
        // dbg!(line);
        let AvSLP = line.parse().map_err(|_| "Couldn't parse AvSLP as f32")?;

        Ok(WeatherDataEntry {
            Dy,
            MxT,
            MnT,
            AvT,
            HDDay,
            AvDP,
            OneHrP: ONE_HR_P,
            TPcpn,
            WxType,
            PDir,
            AvSp,
            Dir,
            MxS,
            SkyC,
            MxR,
            MnR,
            AvSLP,
        })
    }
}

impl FromStr for AsteriskBound<u8> {
    type Err = &'static str;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let value = entry.trim_end_matches('*').parse::<u8>().map_err(|_| "Could not parse value!")?;
        Ok( if entry.contains('*') { AsteriskBound::Untrustworthy(value) } else { AsteriskBound::Clear(value) } )
    }
}

#[derive(Debug)]
enum AsteriskBound<T> {
    Clear(T),
    Untrustworthy(T)    
}
impl<T> AsteriskBound<T> {
    fn val(&self) -> &T {
        match self {
            AsteriskBound::Clear(v) => v,
            AsteriskBound::Untrustworthy(v) => v,
        }
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum WxType {
    R,
    T,
    F,
    H
}

#[allow(unused, non_snake_case)]
#[derive(Debug)]
struct WeatherDataEntry {
    Dy: u8, // interpret this as "day", in the context of each line being a day in a month
    MxT: AsteriskBound<u8>, // minimum temperature
    MnT: AsteriskBound<u8>, // maximum temperature
    AvT: f32, // average temperature
    HDDay: Option<u8>, // unsure?
    AvDP: f32, // unsure?
    OneHrP: (), // unsure? empty field
    TPcpn: f32, // unsure, but always "0.00"
    WxType: HashSet<WxType>, // unsure? comes with specific letters in order
    PDir: u16, // unsure?
    AvSp: f32, // unsure? average something?
    Dir: u16, // unsure?
    MxS: AsteriskBound<u8>, // unsure? maximum something?
    SkyC: f32, // unsure?
    MxR: u8, // unsure? maximum something?
    MnR: u8, // unsure? minimum something?
    AvSLP: f32 // unsure? average something?
}
