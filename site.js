const canvas = document.getElementById("space");
const ctx = canvas.getContext("2d", { alpha: true });

const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
let width = 0;
let height = 0;
let stars = [];
let comets = [];
let animationFrame = 0;
let resizeTimer = 0;
let running = false;

function resize() {
  const ratio = Math.min(window.devicePixelRatio || 1, width >= 1200 ? 1.5 : 2);
  width = window.innerWidth;
  height = window.innerHeight;
  canvas.width = Math.floor(width * ratio);
  canvas.height = Math.floor(height * ratio);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  ctx.setTransform(ratio, 0, 0, ratio, 0, 0);

  const desktop = width >= 1024;
  const density = prefersReducedMotion ? 16000 : desktop ? 8800 : 6800;
  const cap = prefersReducedMotion ? 80 : desktop ? 180 : 210;
  const starCount = Math.min(cap, Math.floor((width * height) / density));
  stars = Array.from({ length: starCount }, () => makeStar(true));
  comets = Array.from({ length: prefersReducedMotion ? 0 : desktop ? 2 : 3 }, (_, idx) => makeComet(idx));
}

function scheduleResize() {
  window.clearTimeout(resizeTimer);
  resizeTimer = window.setTimeout(resize, 120);
}

function makeStar(randomizeY = false) {
  return {
    x: Math.random() * width,
    y: randomizeY ? Math.random() * height : -10,
    r: Math.random() * 1.8 + 0.25,
    vx: (Math.random() - 0.5) * 0.12,
    vy: Math.random() * 0.28 + 0.04,
    hue: Math.random() * 360,
    pulse: Math.random() * Math.PI * 2,
  };
}

function makeComet(idx = 0) {
  return {
    x: Math.random() * width,
    y: Math.random() * height * 0.55,
    vx: 0.65 + idx * 0.2,
    vy: 0.18 + idx * 0.05,
    hue: [190, 285, 122][idx % 3],
    delay: Math.random() * 800,
  };
}

function drawNebula(time) {
  const gradient = ctx.createRadialGradient(
    width * (0.5 + Math.sin(time * 0.00008) * 0.08),
    height * 0.32,
    30,
    width * 0.5,
    height * 0.48,
    Math.max(width, height) * 0.75,
  );
  gradient.addColorStop(0, "rgba(39, 248, 255, 0.10)");
  gradient.addColorStop(0.35, "rgba(255, 61, 242, 0.055)");
  gradient.addColorStop(0.7, "rgba(83, 255, 133, 0.035)");
  gradient.addColorStop(1, "rgba(0, 0, 0, 0)");
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);
}

function drawStar(star, time) {
  if (!prefersReducedMotion) {
    star.x += star.vx;
    star.y += star.vy;
    star.pulse += 0.018;
    star.hue = (star.hue + 0.08) % 360;
  }

  if (star.y > height + 10 || star.x < -10 || star.x > width + 10) {
    Object.assign(star, makeStar(false));
  }

  const alpha = 0.38 + Math.sin(star.pulse + time * 0.001) * 0.28;
  ctx.beginPath();
  ctx.fillStyle = `hsla(${star.hue}, 95%, 72%, ${Math.max(0.18, alpha)})`;
  ctx.shadowBlur = 12;
  ctx.shadowColor = `hsla(${star.hue}, 95%, 62%, 0.9)`;
  ctx.arc(star.x, star.y, star.r, 0, Math.PI * 2);
  ctx.fill();
  ctx.shadowBlur = 0;
}

function drawComet(comet) {
  comet.delay -= 1;
  if (comet.delay > 0) return;

  comet.x += comet.vx;
  comet.y += comet.vy;

  const trail = 92;
  const gradient = ctx.createLinearGradient(comet.x, comet.y, comet.x - trail, comet.y - trail * 0.28);
  gradient.addColorStop(0, `hsla(${comet.hue}, 100%, 70%, 0.95)`);
  gradient.addColorStop(1, `hsla(${comet.hue}, 100%, 70%, 0)`);

  ctx.strokeStyle = gradient;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(comet.x, comet.y);
  ctx.lineTo(comet.x - trail, comet.y - trail * 0.28);
  ctx.stroke();

  if (comet.x > width + 130 || comet.y > height + 80) {
    comet.x = -40 - Math.random() * width * 0.3;
    comet.y = Math.random() * height * 0.45;
    comet.delay = 200 + Math.random() * 900;
  }
}

function frame(time) {
  if (!running) return;

  ctx.clearRect(0, 0, width, height);
  drawNebula(time);

  for (const star of stars) {
    drawStar(star, time);
  }

  if (!prefersReducedMotion) {
    for (const comet of comets) {
      drawComet(comet);
    }
  }

  animationFrame = requestAnimationFrame(frame);
}

function startAnimation() {
  if (running) return;
  running = true;
  animationFrame = requestAnimationFrame(frame);
}

function stopAnimation() {
  running = false;
  cancelAnimationFrame(animationFrame);
}

function handleVisibilityChange() {
  if (document.hidden) {
    stopAnimation();
  } else {
    startAnimation();
  }
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
  if (prefersReducedMotion || !("IntersectionObserver" in window)) {
    revealEls.forEach((el) => el.classList.add("is-visible"));
    return;
  }

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          entry.target.classList.add("is-visible");
          observer.unobserve(entry.target);
        }
      }
    },
    { threshold: 0.14 },
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
    "stable: v4.4.0",
    "previous stable: v4.3.0",
    "next edge: v5.0.0",
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
    "  2. use v4.3.0 for stable or v5.0.0 for active edge development",
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

resize();
setupNavigation();
setupReveals();
setupTerminalDemo();
startAnimation();
window.addEventListener("resize", scheduleResize, { passive: true });
document.addEventListener("visibilitychange", handleVisibilityChange);
window.addEventListener("pagehide", stopAnimation);
