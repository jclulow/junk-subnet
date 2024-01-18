use std::str::FromStr;

use anyhow::{anyhow, Result};
use ipnet::Ipv4Net;

fn split_up(net: Ipv4Net, subtract: Ipv4Net) -> Result<()> {
    let conflict = (net.network() >= subtract.network()
        && net.network() <= subtract.broadcast())
        || (net.broadcast() >= subtract.network()
            && net.broadcast() <= subtract.broadcast());

    if conflict {
        let npfx = net.prefix_len() + 1;
        if npfx > net.max_prefix_len() {
            return Ok(());
        }

        for sub in net.subnets(npfx)? {
            split_up(sub, subtract)?;
        }
    } else {
        println!(
            "{:<13} - {:<15}       {:<17} reserved",
            net.network().to_string(),
            net.broadcast().to_string(),
            net.to_string(),
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let argv = std::env::args().skip(1).collect::<Vec<_>>();

    let base = Ipv4Net::from_str(argv.get(0).ok_or_else(|| anyhow!("base?"))?)?;
    println!("base =     {base}");
    println!("base min = {}", base.network());
    println!("base max = {}", base.broadcast());

    println!();

    let sub = Ipv4Net::from_str(argv.get(1).ok_or_else(|| anyhow!("sub?"))?)?;
    println!("subtract = {sub}");
    println!("sub min =  {}", sub.network());
    println!("sub max =  {}", sub.broadcast());

    println!();

    split_up(base, sub)?;

    Ok(())
}
