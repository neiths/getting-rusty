use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

const MAX_PORT: u16 = 65535;
const DEFAULT_THREADS: u16 = 4;
const TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Debug, PartialEq, Eq)]
struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }

        // Check for help flag anywhere in arguments
        if args.iter().any(|arg| arg == "-h" || arg == "--help") {
            return Err("help".to_string());
        }

        match args.len() {
            // Usage: ip_sniffer <IP>
            2 => {
                let ipaddr = args[1]
                    .parse::<IpAddr>()
                    .map_err(|_| "not a valid IP address; must be IPv4 or IPv6".to_string())?;

                Ok(Arguments {
                    ipaddr,
                    threads: DEFAULT_THREADS,
                })
            }
            // Usage: ip_sniffer -j <THREADS> <IP>
            4 => {
                if args[1] != "-j" && args[1] != "--threads" {
                    return Err(format!("unknown flag '{}', expected '-j' or '--threads'", args[1]));
                }

                let threads = args[2]
                    .parse::<u16>()
                    .map_err(|_| "failed to parse thread number".to_string())?;

                if threads == 0 {
                    return Err("thread count must be greater than 0".to_string());
                }

                let ipaddr = args[3]
                    .parse::<IpAddr>()
                    .map_err(|_| "not a valid IP address; must be IPv4 or IPv6".to_string())?;

                Ok(Arguments { ipaddr, threads })
            }
            _ => Err("invalid number of arguments".to_string()),
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        let socket = SocketAddr::new(addr, port);
        if TcpStream::connect_timeout(&socket, TIMEOUT).is_ok() {
            print!(".");
            let _ = io::stdout().flush();
            let _ = tx.send(port);
        }

        match port.checked_add(num_threads) {
            Some(next) => port = next,
            None => break,
        }
    }
}

fn print_help(program: &str) {
    println!(
        "Usage: {program} [OPTIONS] <IP_ADDRESS>\n\n\
         Scans TCP ports 1 to {MAX_PORT}.\n\n\
         Options:\n  \
           -j, --threads <NUMBER>  Number of concurrent threads (default: {DEFAULT_THREADS})\n  \
           -h, --help              Show this help message\n\n\
         Examples:\n  \
           {program} 127.0.0.1\n  \
           {program} -j 100 192.168.1.1"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err == "help" {
            print_help(program);
            process::exit(0);
        } else {
            eprintln!("{program}: error: {err}\n");
            print_help(program);
            process::exit(1);
        }
    });

    println!(
        "Scanning {} using {} threads (timeout: {:?})...",
        arguments.ipaddr, arguments.threads, TIMEOUT
    );

    let (tx, rx) = channel();
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;

    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    // Drop the initial sender so rx closes once all worker threads complete
    drop(tx);

    let mut open_ports: Vec<u16> = rx.into_iter().collect();
    open_ports.sort_unstable();

    println!();
    for port in &open_ports {
        println!("{port} is open");
    }

    println!("Scan complete. Found {} open port(s).", open_ports.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_help_flag() {
        let args = vec!["ip_sniffer".into(), "-h".into()];
        assert_eq!(Arguments::new(&args), Err("help".to_string()));

        let args_long = vec!["ip_sniffer".into(), "--help".into()];
        assert_eq!(Arguments::new(&args_long), Err("help".to_string()));
    }

    #[test]
    fn test_default_threads() {
        let args = vec!["ip_sniffer".into(), "127.0.0.1".into()];
        let parsed = Arguments::new(&args).unwrap();
        assert_eq!(parsed.ipaddr, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(parsed.threads, DEFAULT_THREADS);
    }

    #[test]
    fn test_custom_threads() {
        let args = vec![
            "ip_sniffer".into(),
            "-j".into(),
            "50".into(),
            "192.168.1.1".into(),
        ];
        let parsed = Arguments::new(&args).unwrap();
        assert_eq!(parsed.ipaddr, "192.168.1.1".parse::<IpAddr>().unwrap());
        assert_eq!(parsed.threads, 50);
    }

    #[test]
    fn test_zero_threads_fails() {
        let args = vec![
            "ip_sniffer".into(),
            "-j".into(),
            "0".into(),
            "127.0.0.1".into(),
        ];
        assert!(Arguments::new(&args).is_err());
    }

    #[test]
    fn test_invalid_ip_fails() {
        let args = vec!["ip_sniffer".into(), "999.999.999.999".into()];
        assert!(Arguments::new(&args).is_err());
    }

    #[test]
    fn test_port_coverage() {
        // Verify stride math hits all ports 1..=MAX_PORT without missing or duplicating
        let num_threads: u16 = 4;
        let mut seen = HashSet::new();

        for i in 0..num_threads {
            let mut port: u16 = i + 1;
            loop {
                assert!(seen.insert(port), "Duplicate port detected: {port}");
                match port.checked_add(num_threads) {
                    Some(next) => port = next,
                    None => break,
                }
            }
        }

        assert_eq!(seen.len(), MAX_PORT as usize);
        assert!(seen.contains(&1));
        assert!(seen.contains(&MAX_PORT));
    }
}
