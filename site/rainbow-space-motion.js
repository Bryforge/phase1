(() => {
  if (window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  if (document.querySelector('.rainbow-space-field')) return;

  const field = document.createElement('div');
  field.className = 'rainbow-space-field';
  field.setAttribute('aria-hidden', 'true');

  const colors = ['#ffffff', '#30f7ff', '#6388ff', '#a45dff', '#ff48dc', '#ff9c35', '#ffe66b', '#66ff9a'];
  const stars = 76;
  for (let i = 0; i < stars; i += 1) {
    const star = document.createElement('span');
    const size = i % 11 === 0 ? 3.4 : i % 7 === 0 ? 2.6 : 1.6;
    star.className = `rs-star${size > 3 ? ' big' : ''}`;
    star.style.setProperty('--x', `${Math.random() * 100}%`);
    star.style.setProperty('--y', `${Math.random() * 100}%`);
    star.style.setProperty('--s', `${size}px`);
    star.style.setProperty('--c', colors[i % colors.length]);
    star.style.setProperty('--o', `${0.46 + Math.random() * 0.48}`);
    star.style.setProperty('--d', `${2.8 + Math.random() * 4.8}s`);
    star.style.setProperty('--delay', `${-Math.random() * 7}s`);
    field.appendChild(star);
  }

  const shooting = [
    ['6%', '8%', '-31deg', '240px', '#30f7ff', '#ff48dc', '6.8s', '-.4s'],
    ['28%', '2%', '-34deg', '210px', '#ffe66b', '#30f7ff', '8.5s', '-3.1s'],
    ['62%', '12%', '-29deg', '260px', '#ff48dc', '#66ff9a', '9.8s', '-5.4s'],
    ['12%', '34%', '-33deg', '190px', '#6388ff', '#ff9c35', '7.6s', '-6.6s'],
    ['72%', '4%', '-36deg', '220px', '#ffffff', '#a45dff', '11s', '-8s'],
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
