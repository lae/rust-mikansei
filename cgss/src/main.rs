extern crate rand;
extern crate rustc_serialize as serialize;
extern crate itertools;
extern crate crypto;

mod cryptographer;
mod network;
mod cryptaes;

fn main() {
    let key = cryptographer::generate_key_string();
    println!("{:?}", key);
    let udid: String = "002415A772A263;172B336p243=835@881>6377317A437:582k354@2817167>143=582=158<7637186B753<117:254n6617781=541o288B733;878k652l872@657@325?363C158:462B817547614552518114128682222435583".to_string();
    //let request: String = "akKAdPcdyQJ9elVQ9meMGJrh55BQaPz+0FMcSDaNtAaF8AcUWun3nXEjGr5RQhf1Q2Z4r2dkc0TAbcgOQYyZaN51wWFrZbHL3rSMFpJF/SvuFnP9T1sNCURoKnF4A7GDPF+4pNqg7PB4reooirGPlMMH2l3O3/AjY5svvzZuxWbyMVGYb127juClDrG4vJVA1Tui+LNhCnokt5KkPKFA8U16aGhZelV6T0RFNU1qWTJZakZoWXpVd01EYzBNR1l6".to_string();
    let request: String = "tcpTjHKJEoz4/F7da71ga58ePwgrnCPTj/HSVRRCqDWkRp5y7gAc5RKb+0vxZ0lTG22UX1Is3L/htn9ODqCOCs+8mTaNCP0Roz5XuA+qOrDD8wt2P1k7EwxuEX2sxoHc4Slku3yLfEVb/wbpByexd4Xdq+VO2jb7fw+qJSrSdaTYJRE835x0TmJPkioOOxD90jV1NYmf58RYaPNg+E9xhVlqRXpZekU1TXpsaFpHVTVNalF4WXprd1lqazFZV1l6".to_string();
    println!("{:?}", network::unpack(request, udid));
}
