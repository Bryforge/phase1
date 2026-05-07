const search = document.getElementById("docs-search");
const searchable = Array.from(document.querySelectorAll("[data-search]"));
const sections = Array.from(document.querySelectorAll(".doc-section"));

search?.addEventListener("input", () => {
  const query = search.value.trim().toLowerCase();
  for (const item of searchable) {
    const text = `${item.textContent} ${item.dataset.search || ""}`.toLowerCase();
    item.classList.toggle("hidden-by-search", query.length > 0 && !text.includes(query));
  }
  for (const section of sections) {
    const text = `${section.textContent} ${section.dataset.search || ""}`.toLowerCase();
    section.classList.toggle("hidden-by-search", query.length > 0 && !text.includes(query));
  }
});

const canvas = document.getElementById("docs-space");
const ctx = canvas.getContext("2d", { alpha: true });
let width = 0;
let height = 0;
let stars = [];

function resize() {
  const ratio = Math.min(window.devicePixelRatio || 1, 2);
  width = window.innerWidth;
  height = window.innerHeight;
  canvas.width = Math.floor(width * ratio);
  canvas.height = Math.floor(height * ratio);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
  stars = Array.from({ length: Math.min(180, Math.floor((width * height) / 7600)) }, () => ({
    x: Math.random() * width,
    y: Math.random() * height,
    r: Math.random() * 1.6 + 0.3,
    vy: Math.random() * 0.2 + 0.04,
    hue: Math.random() * 360,
    pulse: Math.random() * Math.PI * 2,
  }));
}

function draw(time) {
  ctx.clearRect(0, 0, width, height);
  for (const star of stars) {
    star.y += star.vy;
    star.pulse += 0.02;
    star.hue = (star.hue + 0.05) % 360;
    if (star.y > height + 8) {
      star.y = -8;
      star.x = Math.random() * width;
    }
    const alpha = 0.28 + Math.sin(star.pulse + time * 0.001) * 0.22;
    ctx.beginPath();
    ctx.fillStyle = `hsla(${star.hue}, 95%, 72%, ${Math.max(0.15, alpha)})`;
    ctx.shadowBlur = 10;
    ctx.shadowColor = `hsla(${star.hue}, 95%, 62%, 0.8)`;
    ctx.arc(star.x, star.y, star.r, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.shadowBlur = 0;
  requestAnimationFrame(draw);
}

resize();
requestAnimationFrame(draw);
window.addEventListener("resize", resize);
