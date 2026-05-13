(() => {
  const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (prefersReducedMotion || document.querySelector(".phase1-visual-layer")) return;

  const layer = document.createElement("div");
  layer.className = "phase1-visual-layer";
  layer.setAttribute("aria-hidden", "true");

  const width = Math.max(window.innerWidth || 0, 360);
  const starCount = Math.min(width < 760 ? 46 : 92, Math.floor(width / 12));
  const orbCount = width < 760 ? 5 : 9;
  const shootingCount = width < 760 ? 2 : 5;

  for (let i = 0; i < starCount; i += 1) {
    const star = document.createElement("i");
    star.className = "phase1-star";
    star.style.setProperty("--x", `${Math.random() * 100}%`);
    star.style.setProperty("--y", `${Math.random() * 100}%`);
    star.style.setProperty("--s", `${Math.random() * 2.2 + 1}px`);
    star.style.setProperty("--h", `${Math.floor(Math.random() * 360)}`);
    star.style.setProperty("--d", `${Math.random() * 3.5 + 2.4}s`);
    layer.appendChild(star);
  }

  for (let i = 0; i < orbCount; i += 1) {
    const orb = document.createElement("i");
    orb.className = "phase1-orb";
    orb.style.setProperty("--x", `${Math.random() * 100}%`);
    orb.style.setProperty("--y", `${Math.random() * 100}%`);
    orb.style.setProperty("--s", `${Math.random() * 9 + 8}px`);
    orb.style.setProperty("--h", `${[126, 185, 210, 286, 322][i % 5]}`);
    orb.style.setProperty("--d", `${Math.random() * 7 + 7}s`);
    layer.appendChild(orb);
  }

  for (let i = 0; i < shootingCount; i += 1) {
    const shot = document.createElement("i");
    shot.className = "phase1-shooting-star";
    shot.style.setProperty("--x", `${Math.random() * 80 - 20}%`);
    shot.style.setProperty("--y", `${Math.random() * 52 + 4}%`);
    shot.style.setProperty("--l", `${Math.random() * 110 + 130}px`);
    shot.style.setProperty("--h", `${[188, 126, 286, 212, 58][i % 5]}`);
    shot.style.setProperty("--a", `${Math.random() * 10 + 13}deg`);
    shot.style.setProperty("--d", `${Math.random() * 8 + 10}s`);
    shot.style.setProperty("--delay", `${Math.random() * 9}s`);
    layer.appendChild(shot);
  }

  document.body.prepend(layer);
})();
