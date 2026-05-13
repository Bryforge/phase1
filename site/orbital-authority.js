(() => {
  if (window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  if (document.querySelector('.orbital-system-field')) return;

  const field = document.createElement('div');
  field.className = 'orbital-system-field';
  field.setAttribute('aria-hidden', 'true');

  const sun = document.createElement('span');
  sun.className = 'oa-sun';
  field.appendChild(sun);

  const orbits = [
    { size: '290px', offset: '-38px', tilt: '-18deg', p: '6px', color: '#52f1ff', delay: '0s' },
    { size: '440px', offset: '-112px', tilt: '-22deg', p: '8px', color: '#9a7cff', delay: '-4s' },
    { size: '620px', offset: '-205px', tilt: '-15deg', p: '7px', color: '#ffb447', delay: '-8s' },
    { size: '820px', offset: '-304px', tilt: '-24deg', p: '5px', color: '#6dff9a', delay: '-12s' }
  ];

  for (const item of orbits) {
    const orbit = document.createElement('span');
    orbit.className = 'oa-orbit';
    orbit.style.setProperty('--size', item.size);
    orbit.style.setProperty('--offset', item.offset);
    orbit.style.setProperty('--tilt', item.tilt);
    orbit.style.animationDelay = item.delay;

    const planet = document.createElement('span');
    planet.className = 'oa-planet';
    planet.style.setProperty('--p', item.p);
    planet.style.setProperty('--color', item.color);
    orbit.appendChild(planet);
    field.appendChild(orbit);
  }

  document.body.prepend(field);
})();
