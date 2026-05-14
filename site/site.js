// Phase1 public site behavior.
// Keep this file defensive: the recovered homepage no longer requires every legacy
// animation node to exist, so missing optional elements must never break rendering.

const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
const canvas = document.getElementById("space");
const ctx = canvas ? canvas.getContext("2d", { alpha: true }) : null;

let width = 0;
let height = 0;
let stars = [];
let animationFrame = 0;
let resizeTimer = 0;
let running = false;

function makeStar(randomizeY = false) {
  return {
    x: Math.random() * width,
    y: randomizeY ? Math.random() * height : -10,
    r: Math.random() * 1.45 + 0.25,
    vx: (Math.random() - 0.5) * 0.035,
    vy: Math.random() * 0.08 + 0.018,
    hue: Math.random() > 0.55 ? 205 : 28,
    pulse: Math.random() * Math.PI * 2,
  };
}

function resize() {
  if (!canvas || !ctx) return;
  width = window.innerWidth;
  height = window.innerHeight;
  const ratio = Math.min(window.devicePixelRatio || 1, width >= 1200 ? 1.25 : 1.6);
  canvas.width = Math.floor(width * ratio);
  canvas.height = Math.floor(height * ratio);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  ctx.setTransform(ratio, 0, 0, ratio, 0, 0);

  const desktop = width >= 1024;
  const density = prefersReducedMotion ? 20000 : desktop ? 16000 : 14000;
  const cap = prefersReducedMotion ? 40 : desktop ? 110 : 90;
  const starCount = Math.min(cap, Math.floor((width * height) / density));
  stars = Array.from({ length: starCount }, () => makeStar(true));
}

function scheduleResize() {
  window.clearTimeout(resizeTimer);
  resizeTimer = window.setTimeout(resize, 120);
}

function drawNebula(time) {
  if (!ctx) return;
  const gradient = ctx.createRadialGradient(
    width * (0.50 + Math.sin(time * 0.00004) * 0.04),
    height * 0.32,
    20,
    width * 0.52,
    height * 0.48,
    Math.max(width, height) * 0.78,
  );
  gradient.addColorStop(0, "rgba(43, 141, 255, 0.08)");
  gradient.addColorStop(0.42, "rgba(255, 122, 24, 0.045)");
  gradient.addColorStop(1, "rgba(0, 0, 0, 0)");
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);
}

function drawStar(star, time) {
  if (!ctx) return;
  if (!prefersReducedMotion) {
    star.x += star.vx;
    star.y += star.vy;
    star.pulse += 0.006;
  }

  if (star.y > height + 10 || star.x < -10 || star.x > width + 10) {
    Object.assign(star, makeStar(false));
  }

  const alpha = 0.22 + Math.sin(star.pulse + time * 0.00045) * 0.16;
  ctx.beginPath();
  ctx.fillStyle = `hsla(${star.hue}, 95%, 72%, ${Math.max(0.10, alpha)})`;
  ctx.shadowBlur = 8;
  ctx.shadowColor = `hsla(${star.hue}, 95%, 62%, 0.45)`;
  ctx.arc(star.x, star.y, star.r, 0, Math.PI * 2);
  ctx.fill();
  ctx.shadowBlur = 0;
}

function frame(time) {
  if (!running || !canvas || !ctx) return;
  ctx.clearRect(0, 0, width, height);
  drawNebula(time);
  for (const star of stars) drawStar(star, time);
  animationFrame = requestAnimationFrame(frame);
}

function startAnimation() {
  if (!canvas || !ctx || running) return;
  running = true;
  animationFrame = requestAnimationFrame(frame);
}

function stopAnimation() {
  running = false;
  cancelAnimationFrame(animationFrame);
}

function handleVisibilityChange() {
  if (document.hidden) stopAnimation();
  else startAnimation();
}

function setupNavigation() {
  const toggle = document.querySelector(".nav-toggle");
  const links = document.getElementById("nav-links");
  if (!toggle || !links) return;

  toggle.addEventListener("click", () => {
    const expanded = toggle.getAttribute("aria-expanded") === "true";
    toggle.setAttribute("aria-expanded", String(!expanded));
    links.classList.toggle("is-open", !expanded);
  });

  links.addEventListener("click", (event) => {
    if (event.target instanceof HTMLAnchorElement) {
      toggle.setAttribute("aria-expanded", "false");
      links.classList.remove("is-open");
    }
  });
}

function setupReveals() {
  const revealEls = Array.from(document.querySelectorAll(".reveal"));
  if (!revealEls.length) return;

  // Fail-open for public representation: content should never be invisible because
  // animation setup failed, loaded late, or a browser blocks observers.
  revealEls.forEach((el) => el.classList.add("is-visible"));

  if (prefersReducedMotion || !("IntersectionObserver" in window)) return;

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          entry.target.classList.add("is-visible");
          observer.unobserve(entry.target);
        }
      }
    },
    { threshold: 0.10 },
  );

  revealEls.forEach((el) => observer.observe(el));
}

const demoResponses = {
  help: [
    "available demo commands:",
    "  help        show command list",
    "  version     show phase1 version track",
    "  sysinfo     inspect simulated system profile",
    "  wiki-quick  open the quick operator guide",
    "  security    show safe-mode posture",
  ].join("\n"),
  version: [
    "phase1 // advanced operator kernel",
    "stable: v6.0.0",
    "previous stable: v5.0.0",
    "next edge: v7.0.1",
    "compatibility base: v3.6.0",
    "language: Rust",
  ].join("\n"),
  sysinfo: [
    "system: phase1 virtual OS console",
    "kernel: simulated operator kernel",
    "vfs: online",
    "process table: available",
    "audit log: enabled",
    "browser: guarded",
  ].join("\n"),
  "wiki-quick": [
    "wiki-quick:",
    "  1. clone the repo",
    "  2. use v6.0.0 for stable or v7.0.1 for active edge development",
    "  3. run cargo run",
    "  4. type help, security, sysinfo, wiki",
    "  5. keep safe mode on unless you trust the host workflow",
  ].join("\n"),
  security: [
    "security posture:",
    "  safe mode: on by default",
    "  host tools: guarded",
    "  secrets: never required for normal use",
    "  values: secure · private · powerful · open",
  ].join("\n"),
  clear: "",
};

function setupTerminalDemo() {
  const form = document.getElementById("terminal-form");
  const input = document.getElementById("terminal-input");
  const output = document.getElementById("terminal-output");
  const quickCommands = document.querySelectorAll("[data-command]");
  if (!form || !input || !output) return;

  const history = [];

  const print = (text) => {
    output.textContent = text;
    output.scrollTop = output.scrollHeight;
  };

  const append = (command, response) => {
    if (command === "clear") {
      history.length = 0;
      print("phase1 demo reset. type help to begin.");
      return;
    }

    history.push(`phase1://root ~ # ${command}`);
    history.push(response || `unknown command: ${command}\ntype help for available demo commands.`);
    print(history.slice(-18).join("\n\n"));
  };

  print([
    "phase1 browser console demo",
    "safe mode: on // host tools: guarded",
    "type help, version, sysinfo, wiki-quick, security, or clear",
  ].join("\n"));

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const command = input.value.trim().toLowerCase();
    if (!command) return;
    append(command, demoResponses[command]);
    input.value = "";
  });

  quickCommands.forEach((button) => {
    button.addEventListener("click", () => {
      const command = button.getAttribute("data-command") || "help";
      append(command, demoResponses[command]);
    });
  });
}

async function setupPublicStatus() {
  const summary = document.querySelector("[data-status-summary]");
  const button = document.querySelector("[data-status-button]");
  const overall = document.querySelector("[data-status-overall]");
  const detail = document.querySelector("[data-status-detail]");
  const description = document.querySelector("[data-status-description]");

  if (!summary && !button && !overall && !detail && !description) return;

  try {
    const response = await fetch("status.json", { cache: "no-store" });
    if (!response.ok) throw new Error(`status fetch failed: ${response.status}`);
    const data = await response.json();

    const pct = Number(data.overall_estimated_completion_percent || 0);
    const repo = (data.projects || []).find((project) =>
      String(project.name || "").toLowerCase().includes("repository organization"),
    );
    const repoPct = Number(repo?.estimated_completion_percent || 0);
    const updated = data.last_updated_utc ? ` · updated ${data.last_updated_utc}` : "";

    if (summary) summary.textContent = `Live project status · ${pct}% roadmap · Repository organization ${repoPct}% · View details`;
    if (button) button.textContent = `Live status · ${pct}%`;
    if (overall) overall.textContent = `${pct}%`;
    if (detail) detail.textContent = `${pct}% roadmap · repository organization ${repoPct}%`;
    if (description) {
      description.textContent =
        `Public status marker is live from status.json with repository metrics, estimated project percentages, and non-claim boundaries${updated}.`;
    }
  } catch (error) {
    console.warn("Phase1 public status unavailable:", error);
  }
}

function boot() {
  setupPublicStatus();
  setupNavigation();
  setupReveals();
  setupTerminalDemo();

  if (canvas && ctx) {
    resize();
    startAnimation();
    window.addEventListener("resize", scheduleResize, { passive: true });
    document.addEventListener("visibilitychange", handleVisibilityChange);
    window.addEventListener("pagehide", stopAnimation);
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", boot, { once: true });
} else {
  boot();
}
