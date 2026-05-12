use std::io::{self, Write};

pub const GAME_TITLE: &str = "Phase1 Arena";
pub const GAME_VERSION: &str = "0.1.0";

const WIDTH: usize = 13;
const HEIGHT: usize = 9;
const MAP: [&str; HEIGHT] = [
    "#############",
    "#P..#....M..#",
    "#.#.#.###.#.#",
    "#.#...#...#.#",
    "#.###.#.#.#E#",
    "#.....#.#...#",
    "###.#...#.###",
    "#A..#..M..H.#",
    "#############",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn delta(self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::West => "west",
            Self::East => "east",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Enemy {
    x: usize,
    y: usize,
    hp: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    walls: [[bool; WIDTH]; HEIGHT],
    ammo_pickups: Vec<(usize, usize)>,
    medkits: Vec<(usize, usize)>,
    enemies: Vec<Enemy>,
    player_x: usize,
    player_y: usize,
    exit_x: usize,
    exit_y: usize,
    dir: Dir,
    hp: i32,
    ammo: i32,
    kills: u32,
    done: bool,
    won: bool,
}

impl Game {
    fn new() -> Self {
        let mut walls = [[false; WIDTH]; HEIGHT];
        let mut ammo_pickups = Vec::new();
        let mut medkits = Vec::new();
        let mut enemies = Vec::new();
        let mut player_x = 1;
        let mut player_y = 1;
        let mut exit_x = WIDTH - 2;
        let mut exit_y = HEIGHT - 2;

        for (y, row) in MAP.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    '#' => walls[y][x] = true,
                    'P' => {
                        player_x = x;
                        player_y = y;
                    }
                    'M' => enemies.push(Enemy { x, y, hp: 10 }),
                    'A' => ammo_pickups.push((x, y)),
                    'H' => medkits.push((x, y)),
                    'E' => {
                        exit_x = x;
                        exit_y = y;
                    }
                    _ => {}
                }
            }
        }

        Self {
            walls,
            ammo_pickups,
            medkits,
            enemies,
            player_x,
            player_y,
            exit_x,
            exit_y,
            dir: Dir::East,
            hp: 30,
            ammo: 8,
            kills: 0,
            done: false,
            won: false,
        }
    }

    fn step(&mut self, raw: &str) -> String {
        let command = raw.trim().to_ascii_lowercase();
        let mut out = String::new();
        let acted = match command.as_str() {
            "" | "look" | "map" => {
                out.push_str("You scan the arena.\n");
                false
            }
            "help" | "?" => {
                out.push_str(&help_text());
                false
            }
            "quit" | "exit" => {
                self.done = true;
                out.push_str("Phase1 Arena: session ended.\n");
                false
            }
            "w" | "north" => self.try_move(Dir::North, &mut out),
            "s" | "south" => self.try_move(Dir::South, &mut out),
            "a" | "west" => self.try_move(Dir::West, &mut out),
            "d" | "east" => self.try_move(Dir::East, &mut out),
            "f" | "fire" | "shoot" => self.fire(&mut out),
            _ => {
                out.push_str("Unknown command. Try w/a/s/d, fire, map, help, quit.\n");
                false
            }
        };

        if acted && !self.done {
            self.enemy_turn(&mut out);
            self.check_pickups(&mut out);
            self.check_goal(&mut out);
            if self.hp <= 0 {
                self.done = true;
                self.won = false;
                out.push_str("You are down. Arena run failed.\n");
            }
        }

        out.push_str(&self.render());
        out
    }

    fn try_move(&mut self, dir: Dir, out: &mut String) -> bool {
        self.dir = dir;
        let (dx, dy) = dir.delta();
        let nx = self.player_x as isize + dx;
        let ny = self.player_y as isize + dy;
        if nx < 0 || ny < 0 || nx >= WIDTH as isize || ny >= HEIGHT as isize {
            out.push_str("You hit the arena boundary.\n");
            return true;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if self.walls[ny][nx] {
            out.push_str("A wall blocks your path.\n");
            return true;
        }
        if self
            .enemies
            .iter()
            .any(|enemy| enemy.x == nx && enemy.y == ny)
        {
            out.push_str("A hostile blocks that tile. Fire first.\n");
            return true;
        }
        self.player_x = nx;
        self.player_y = ny;
        out.push_str(&format!("You move {}.\n", dir.name()));
        true
    }

    fn fire(&mut self, out: &mut String) -> bool {
        if self.ammo <= 0 {
            out.push_str("Click. No ammo.\n");
            return true;
        }
        self.ammo -= 1;
        let Some(target_idx) = self.first_enemy_in_sight() else {
            out.push_str("Your shot echoes through the corridor.\n");
            return true;
        };
        self.enemies[target_idx].hp -= 10;
        if self.enemies[target_idx].hp <= 0 {
            let enemy = self.enemies.remove(target_idx);
            self.kills += 1;
            out.push_str(&format!(
                "Direct hit. Hostile cleared at {},{}.\n",
                enemy.x, enemy.y
            ));
        } else {
            out.push_str("Direct hit. The hostile staggers.\n");
        }
        true
    }

    fn first_enemy_in_sight(&self) -> Option<usize> {
        let (dx, dy) = self.dir.delta();
        let mut x = self.player_x as isize;
        let mut y = self.player_y as isize;
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
                return None;
            }
            let ux = x as usize;
            let uy = y as usize;
            if self.walls[uy][ux] {
                return None;
            }
            if let Some(idx) = self
                .enemies
                .iter()
                .position(|enemy| enemy.x == ux && enemy.y == uy)
            {
                return Some(idx);
            }
        }
    }

    fn enemy_turn(&mut self, out: &mut String) {
        let mut occupied = self
            .enemies
            .iter()
            .map(|enemy| (enemy.x, enemy.y))
            .collect::<Vec<_>>();

        for idx in 0..self.enemies.len() {
            let enemy = self.enemies[idx];
            let distance = manhattan(enemy.x, enemy.y, self.player_x, self.player_y);
            if distance == 1 {
                self.hp -= 5;
                out.push_str("A hostile strikes for 5 damage.\n");
                continue;
            }
            if distance > 5 {
                continue;
            }

            let mut candidates = Vec::new();
            if enemy.x < self.player_x {
                candidates.push((enemy.x + 1, enemy.y));
            }
            if enemy.x > self.player_x {
                candidates.push((enemy.x - 1, enemy.y));
            }
            if enemy.y < self.player_y {
                candidates.push((enemy.x, enemy.y + 1));
            }
            if enemy.y > self.player_y {
                candidates.push((enemy.x, enemy.y - 1));
            }

            if let Some((nx, ny)) = candidates.into_iter().find(|(nx, ny)| {
                !self.walls[*ny][*nx]
                    && (*nx, *ny) != (self.player_x, self.player_y)
                    && !occupied
                        .iter()
                        .enumerate()
                        .any(|(other_idx, pos)| other_idx != idx && *pos == (*nx, *ny))
            }) {
                occupied[idx] = (nx, ny);
                self.enemies[idx].x = nx;
                self.enemies[idx].y = ny;
            }
        }
    }

    fn check_pickups(&mut self, out: &mut String) {
        if let Some(idx) = self
            .ammo_pickups
            .iter()
            .position(|pos| *pos == (self.player_x, self.player_y))
        {
            self.ammo_pickups.remove(idx);
            self.ammo += 6;
            out.push_str("Ammo cache loaded: +6 shells.\n");
        }
        if let Some(idx) = self
            .medkits
            .iter()
            .position(|pos| *pos == (self.player_x, self.player_y))
        {
            self.medkits.remove(idx);
            self.hp = (self.hp + 10).min(30);
            out.push_str("Medkit applied: +10 health.\n");
        }
    }

    fn check_goal(&mut self, out: &mut String) {
        if (self.player_x, self.player_y) == (self.exit_x, self.exit_y) {
            if self.enemies.is_empty() {
                self.done = true;
                self.won = true;
                out.push_str("Exit reached. Phase1 Arena cleared.\n");
            } else {
                out.push_str("The exit is sealed until every hostile is cleared.\n");
            }
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("\nphase1 arena // clean-room text-mode game\n");
        out.push_str(
            "asset mode: original ASCII, no external WAD or proprietary assets required\n",
        );
        out.push_str(&format!(
            "hp={} ammo={} dir={} kills={} enemies={} status={}\n",
            self.hp.max(0),
            self.ammo,
            self.dir.name(),
            self.kills,
            self.enemies.len(),
            if self.done {
                if self.won {
                    "cleared"
                } else {
                    "ended"
                }
            } else {
                "live"
            }
        ));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ch = if (x, y) == (self.player_x, self.player_y) {
                    '@'
                } else if self
                    .enemies
                    .iter()
                    .any(|enemy| enemy.x == x && enemy.y == y)
                {
                    'M'
                } else if self.ammo_pickups.iter().any(|pos| *pos == (x, y)) {
                    'A'
                } else if self.medkits.iter().any(|pos| *pos == (x, y)) {
                    'H'
                } else if (x, y) == (self.exit_x, self.exit_y) {
                    'E'
                } else if self.walls[y][x] {
                    '#'
                } else {
                    '.'
                };
                out.push(ch);
            }
            out.push('\n');
        }
        out
    }
}

pub fn run(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("start" | "play") => help_text(),
        Some("help") | Some("--help") | Some("-h") => help_text(),
        Some("demo") => run_script(&["d", "d", "d", "fire", "map"]),
        Some("script") => {
            let commands = args[1..].iter().map(String::as_str).collect::<Vec<_>>();
            run_script(&commands)
        }
        Some("roadmap") => game_roadmap(),
        Some("dev") | Some("workspace") => game_workspace(),
        Some("test-plan") | Some("tests") => game_test_plan(),
        Some(other) => format!("arena: unknown option '{other}'\n{}", help_text()),
    }
}

pub fn game(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status" | "workspace") => game_workspace(),
        Some("arena") => run(&args[1..]),
        Some("roadmap") => game_roadmap(),
        Some("files") => game_files(),
        Some("test-plan") | Some("tests") => game_test_plan(),
        Some("version") => format!("{GAME_TITLE} {GAME_VERSION}\n"),
        Some("help") | Some("--help") | Some("-h") => game_help(),
        Some(other) => format!("game: unknown option '{other}'\n{}", game_help()),
    }
}

pub fn play() {
    let mut game = Game::new();
    println!("{}", help_text());
    println!("{}", game.render());

    while !game.done {
        print!("arena> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => {
                println!("Phase1 Arena: input closed.");
                break;
            }
            Ok(_) => println!("{}", game.step(&input)),
        }
    }
}

fn run_script(commands: &[&str]) -> String {
    let mut game = Game::new();
    let mut out = String::from("phase1 arena scripted run\n");
    out.push_str(&game.render());
    for command in commands {
        if game.done {
            break;
        }
        out.push_str(&format!("\narena> {command}\n"));
        out.push_str(&game.step(command));
    }
    out
}

fn help_text() -> String {
    format!(
        "{GAME_TITLE}\nusage : arena [start|demo|script <commands...>|roadmap|dev|test-plan|help]\nplay  : run `arena start`, then use w/a/s/d, fire, map, help, quit\ngoal  : clear hostiles, grab supplies, and reach E\nnote  : original ASCII arena; no proprietary game assets or WAD files are used\n"
    )
}

fn game_help() -> String {
    "phase1 game workspace\nusage : game [status|files|roadmap|test-plan|version|help]\nplay  : arena start\nfocus : sh scripts/test-game.sh\n".to_string()
}

fn game_workspace() -> String {
    format!(
        "phase1 game workspace\nactive game : {GAME_TITLE}\nversion     : {GAME_VERSION}\ncommand     : arena start\ndev docs    : docs/developers/GAME_DEV.md\nrunner      : sh scripts/test-game.sh\nscope       : game-only module, game integration tests, and WASI-lite launcher wiring\n"
    )
}

fn game_files() -> String {
    "phase1 game files\n- src/arena.rs\n- tests/game.rs\n- docs/developers/GAME_DEV.md\n- scripts/test-game.sh\n- src/wasm.rs integration\n- src/registry.rs command metadata\n".to_string()
}

fn game_roadmap() -> String {
    "phase1 game roadmap\ncomplete : rename openDoom prototype to Phase1 Arena\ncomplete : isolate game logic in src/arena.rs\ncomplete : add game-only dev docs and test runner\ncomplete : wire arena through direct command and WASI-lite launcher\nnext     : add levels, score persistence, and terminal-friendly combat tuning\nplanned  : optional color sprites and mobile-sized arena layouts\n".to_string()
}

fn game_test_plan() -> String {
    "phase1 game test plan\n  cargo fmt --all -- --check\n  cargo test --bin phase1 arena::\n  cargo test --test game -- --nocapture\n  cargo test --test bleeding -- --nocapture\n".to_string()
}

fn manhattan(ax: usize, ay: usize, bx: usize, by: usize) -> usize {
    ax.abs_diff(bx) + ay.abs_diff(by)
}

#[cfg(test)]
mod tests {
    use super::{game, run, Game};

    #[test]
    fn demo_renders_phase1_arena() {
        let out = run(&["demo".to_string()]);
        assert!(out.contains("phase1 arena"));
        assert!(out.contains("@"));
        assert!(out.contains("M"));
        assert!(out.contains("asset mode: original ASCII"));
    }

    #[test]
    fn fire_clears_visible_enemy() {
        let mut game = Game::new();
        game.player_x = 8;
        game.player_y = 1;
        game.dir = super::Dir::East;
        let out = game.step("fire");
        assert!(out.contains("Direct hit"));
        assert_eq!(game.kills, 1);
    }

    #[test]
    fn game_workspace_reports_focused_files() {
        let out = game(&["files".to_string()]);
        assert!(out.contains("src/arena.rs"));
        assert!(out.contains("tests/game.rs"));
        assert!(out.contains("docs/developers/GAME_DEV.md"));
    }
}
