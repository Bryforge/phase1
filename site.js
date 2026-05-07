const canvas = document.getElementById("space");
const ctx = canvas.getContext("2d", { alpha: true });

const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
let width = 0;
let height = 0;
let stars = [];
let comets = [];

function resize() {
  const ratio = Math.min(window.devicePixelRatio || 1, 2);
  width = window.innerWidth;
  height = window.innerHeight;
  canvas.width = Math.floor(width * ratio);
  canvas.height = Math.floor(height * ratio);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  ctx.setTransform(ratio, 0, 0, ratio, 0, 0);

  const starCount = Math.min(220, Math.floor((width * height) / 6200));
  stars = Array.from({ length: starCount }, () => makeStar(true));
  comets = Array.from({ length: 3 }, (_, idx) => makeComet(idx));
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
  star.x += star.vx;
  star.y += star.vy;
  star.pulse += 0.018;
  star.hue = (star.hue + 0.08) % 360;

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

  requestAnimationFrame(frame);
}

resize();
requestAnimationFrame(frame);
window.addEventListener("resize", resize);
