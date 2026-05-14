use crate::commands::Phase1Shell;
use crate::kernel::VfsNode;

const REGISTRY_KEY: &str = "PHASE1_ANALYSIS_REGISTRY";

#[derive(Clone, Debug, Eq, PartialEq)]
struct AnalysisRecord {
    id: String,
    path: String,
    size_bytes: usize,
    sha256: String,
}

pub fn run(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") | Some("help") | Some("--help") | Some("-h") => status(),
        Some("load") => load(shell, &args[1..]),
        Some("inspect") => inspect(shell, &args[1..]),
        Some("report") => planned("report"),
        Some("forget") => planned("forget"),
        Some(other) => format!(
            "phase1 analysis\nstatus           : unknown-action\naction           : {other}\nmode             : no-execute\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\nhelp             : analyze status | analyze load <path> | analyze inspect <id>\n"
        ),
    }
}

fn status() -> String {
    concat!(
        "phase1 analysis\n",
        "status           : experimental\n",
        "mode             : metadata-only\n",
        "execution-state  : not-executed\n",
        "host-execution   : disabled\n",
        "sandbox-claim    : not-claimed\n",
        "static-analysis  : planned\n",
        "dynamic-analysis : future-restricted\n",
        "sample-registry  : session-local\n",
        "reports          : planned\n",
        "fyr-integration  : planned\n",
        "base1-evidence   : planned\n",
        "claim-boundary   : metadata-only-loading\n",
        "usage            : analyze load <path> | analyze inspect <id>\n",
    )
    .to_string()
}

fn planned(action: &str) -> String {
    format!(
        "phase1 analysis\nstatus           : planned\naction           : {action}\nmode             : no-execute\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\nclaim-boundary   : metadata-only-loading\n"
    )
}

fn load(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first().map(String::as_str) else {
        return concat!(
            "phase1 analysis load\n",
            "status           : missing-path\n",
            "usage            : analyze load <path>\n",
            "mode             : no-execute\n",
            "execution-state  : not-executed\n",
            "host-execution   : disabled\n",
            "sandbox-claim    : not-claimed\n",
        )
        .to_string();
    };

    let resolved = shell.kernel.vfs.resolve_path(path);
    let Some(node) = shell.kernel.vfs.get_node(&resolved) else {
        return format!(
            "phase1 analysis load\nstatus           : missing\npath             : {}\nerror            : no-such-vfs-file\nmode             : no-execute\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\nclaim-boundary   : metadata-only-loading\n",
            resolved.display()
        );
    };

    let VfsNode::File { content, .. } = node else {
        return format!(
            "phase1 analysis load\nstatus           : unsupported\npath             : {}\nerror            : directory-not-sample\nmode             : no-execute\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\nclaim-boundary   : metadata-only-loading\n",
            resolved.display()
        );
    };

    let content = content.clone();
    let bytes = content.as_bytes();
    let digest = sha256_hex(bytes);
    let id = format!("sha256-{}", &digest[..12]);
    let record = AnalysisRecord {
        id: id.clone(),
        path: resolved.display().to_string(),
        size_bytes: bytes.len(),
        sha256: digest.clone(),
    };

    let mut records = load_registry(shell);
    let duplicate = records.iter().any(|item| item.id == id);

    if !duplicate {
        records.push(record.clone());
        save_registry(shell, &records);
    }

    shell.kernel.audit.record(format!(
        "analysis.load id={id} path={} bytes={} state=not-executed",
        resolved.display(),
        bytes.len()
    ));

    format_record(
        "load",
        if duplicate { "duplicate" } else { "loaded" },
        &record,
    )
}

fn inspect(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(id) = args.first().map(String::as_str) else {
        return concat!(
            "phase1 analysis inspect\n",
            "status           : missing-id\n",
            "usage            : analyze inspect <id>\n",
            "mode             : no-execute\n",
            "execution-state  : not-executed\n",
            "host-execution   : disabled\n",
            "sandbox-claim    : not-claimed\n",
            "dynamic-analysis : future-restricted\n",
            "claim-boundary   : metadata-only-loading\n",
        )
        .to_string();
    };

    let records = load_registry(shell);
    let Some(record) = records.iter().find(|item| item.id == id) else {
        return format!(
            "phase1 analysis inspect\nstatus           : missing\nid               : {id}\nerror            : no-such-analysis-record\nmode             : no-execute\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\ndynamic-analysis : future-restricted\nclaim-boundary   : metadata-only-loading\n"
        );
    };

    format_record("inspect", "found", record)
}

fn format_record(action: &str, status: &str, record: &AnalysisRecord) -> String {
    format!(
        "phase1 analysis {action}\nstatus           : {status}\nid               : {}\npath             : {}\nsize-bytes       : {}\nsha256           : {}\nsource           : vfs\nloaded-at        : session\ntrust-state      : untrusted\nexecution-state  : not-executed\nhost-execution   : disabled\nsandbox-claim    : not-claimed\ndynamic-analysis : future-restricted\nclaim-boundary   : metadata-only-loading\n",
        record.id, record.path, record.size_bytes, record.sha256
    )
}

fn load_registry(shell: &Phase1Shell) -> Vec<AnalysisRecord> {
    shell
        .env
        .get(REGISTRY_KEY)
        .map(|raw| {
            raw.lines()
                .filter_map(AnalysisRecord::decode)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn save_registry(shell: &mut Phase1Shell, records: &[AnalysisRecord]) {
    let encoded = records
        .iter()
        .map(AnalysisRecord::encode)
        .collect::<Vec<_>>()
        .join("\n");
    shell.env.insert(REGISTRY_KEY.to_string(), encoded);
}

impl AnalysisRecord {
    fn encode(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            encode_field(&self.id),
            encode_field(&self.path),
            self.size_bytes,
            encode_field(&self.sha256)
        )
    }

    fn decode(raw: &str) -> Option<Self> {
        let mut parts = raw.split('|');
        let id = decode_field(parts.next()?)?;
        let path = decode_field(parts.next()?)?;
        let size_bytes = parts.next()?.parse().ok()?;
        let sha256 = decode_field(parts.next()?)?;
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            id,
            path,
            size_bytes,
            sha256,
        })
    }
}

fn encode_field(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'%' | b'|' | b'\n' | b'\r' => out.push_str(&format!("%{byte:02X}")),
            _ => out.push(byte as char),
        }
    }
    out
}

fn decode_field(value: &str) -> Option<String> {
    let mut out = Vec::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'%' {
            out.push(bytes[index]);
            index += 1;
            continue;
        }
        let high = *bytes.get(index + 1)?;
        let low = *bytes.get(index + 2)?;
        let decoded = hex_value(high)? * 16 + hex_value(low)?;
        out.push(decoded);
        index += 3;
    }
    String::from_utf8(out).ok()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn sha256_hex(input: &[u8]) -> String {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h = H0;
    let bit_len = (input.len() as u64) * 8;
    let mut data = input.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in data.chunks(64) {
        let mut w = [0u32; 64];
        for (i, word) in w.iter_mut().take(16).enumerate() {
            let start = i * 4;
            *word = u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    h.iter().map(|word| format!("{word:08x}")).collect()
}
