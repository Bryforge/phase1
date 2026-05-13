(() => {
  if (window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  if (document.querySelector('.rainbow-space-field')) return;

  const field = document.createElement('div');
  field.className = 'rainbow-space-field';
  field.setAttribute('aria-hidden', 'true');

  const colors = ['#ffffff', '#30f7ff', '#6388ff', '#a45dff', '#ff48dc', '#ff9c35', '#ffe66b', '#66ff9a'];
  const stars = 64;
  for (let i = 0; i < stars; i += 1) {
    const star = document.createElement('span');
    const size = i % 13 === 0 ? 3.0 : i % 8 === 0 ? 2.25 : 1.35;
    star.className = `rs-star${size >= 3 ? ' big' : ''}`;
    star.style.setProperty('--x', `${Math.random() * 100}%`);
    star.style.setProperty('--y', `${Math.random() * 100}%`);
    star.style.setProperty('--s', `${size}px`);
    star.style.setProperty('--c', colors[i % colors.length]);
    star.style.setProperty('--o', `${0.34 + Math.random() * 0.34}`);
    star.style.setProperty('--d', `${8 + Math.random() * 9}s`);
    star.style.setProperty('--delay', `${-Math.random() * 16}s`);
    field.appendChild(star);
  }

  const shooting = [
    ['4%', '8%', '-31deg', '220px', '#30f7ff', '#ff48dc', '21s', '-2s'],
    ['28%', '2%', '-34deg', '190px', '#ffe66b', '#30f7ff', '28s', '-9s'],
    ['62%', '12%', '-29deg', '230px', '#ff48dc', '#66ff9a', '34s', '-17s'],
    ['12%', '34%', '-33deg', '170px', '#6388ff', '#ff9c35', '26s', '-22s'],
  ];

  for (const [x, y, r, w, c1, c2, d, delay] of shooting) {
    const meteor = document.createElement('span');
    meteor.className = 'rs-shooting-star';
    meteor.style.setProperty('--x', x);
    meteor.style.setProperty('--y', y);
    meteor.style.setProperty('--r', r);
    meteor.style.setProperty('--w', w);
    meteor.style.setProperty('--c1', c1);
    meteor.style.setProperty('--c2', c2);
    meteor.style.setProperty('--d', d);
    meteor.style.setProperty('--delay', delay);
    field.appendChild(meteor);
  }

  document.body.prepend(field);
})();
