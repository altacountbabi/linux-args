use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct KernelArgs {
    // Args
    pub root: Option<PathBuf>,
    pub loglevel: Option<u32>,
    pub init: Option<PathBuf>,
    pub mem: Option<usize>,
    pub vga: Option<u32>,

    // Flags
    pub single: bool,
    pub nomodeset: bool,
    pub debug: bool,
    pub noapic: bool,
    pub irqpool: bool,
    pub nolapic: bool,
    pub ipv6_disable: bool,
    pub ro: bool,
    pub quiet: bool,
    pub silent: bool,
    pub splash: bool,
}

impl KernelArgs {
    pub fn parse(cmdline: &str) -> Self {
        let mut args_struct = KernelArgs::default();

        let args: Vec<&str> = cmdline.split_whitespace().collect();
        for arg in args {
            if let Some((key, value)) = parse_key_value(arg) {
                match key.as_str() {
                    "root" => args_struct.root = Some(PathBuf::from(value)),
                    "loglevel" => args_struct.loglevel = value.parse().ok(),
                    "init" => args_struct.init = Some(PathBuf::from(value)),
                    "mem" => args_struct.mem = value.parse().ok(),
                    "vga" => args_struct.vga = value.parse().ok(),
                    "ipv6.disable" => {
                        if value == "1" {
                            args_struct.ipv6_disable = true;
                        }
                    }
                    _ => {}
                }
            } else if let Some(flag) = parse_flag(arg) {
                match flag.as_str() {
                    "quiet" => args_struct.quiet = true,
                    "silent" => args_struct.silent = true,
                    "splash" => args_struct.splash = true,
                    "single" => args_struct.single = true,
                    "nomodeset" => args_struct.nomodeset = true,
                    "debug" => args_struct.debug = true,
                    "noapic" => args_struct.noapic = true,
                    "irqpool" => args_struct.irqpool = true,
                    "nolapic" => args_struct.nolapic = true,
                    "ro" => args_struct.ro = true,
                    _ => {}
                }
            }
        }

        args_struct
    }
}

fn parse_key_value(arg: &str) -> Option<(String, String)> {
    let mut parts = arg.splitn(2, '=');
    match (parts.next(), parts.next()) {
        (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
        _ => None,
    }
}

fn parse_flag(arg: &str) -> Option<String> {
    if !arg.contains('=') {
        Some(arg.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let args = KernelArgs::parse("root=/dev/sda1 init=/sbin/init loglevel=3 mem=8192 vga=732 ipv6.disable=1 quiet silent splash single nomodeset debug noapic irqpool nolapic ro");

        assert_eq!(args.root, Some(PathBuf::from("/dev/sda1")));
        assert_eq!(args.init, Some(PathBuf::from("/sbin/init")));
        assert_eq!(args.loglevel, Some(3));
        assert_eq!(args.mem, Some(8192));
        assert_eq!(args.vga, Some(732));
        assert_eq!(args.ipv6_disable, true);
        assert_eq!(args.quiet, true);
        assert_eq!(args.silent, true);
        assert_eq!(args.splash, true);
        assert_eq!(args.single, true);
        assert_eq!(args.nomodeset, true);
        assert_eq!(args.debug, true);
        assert_eq!(args.noapic, true);
        assert_eq!(args.irqpool, true);
        assert_eq!(args.nolapic, true);
        assert_eq!(args.ro, true);
    }
}
