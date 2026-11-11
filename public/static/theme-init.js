(function () {
  try {
    var t = window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
    var root = document.documentElement;
    root.setAttribute('data-theme', t);
    var fine = window.matchMedia('(min-width: 901px) and (pointer: fine)').matches;
    var reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    if (fine && !reduced) root.classList.add('fx');
  } catch (e) {
    document.documentElement.setAttribute('data-theme', 'dark');
  }
})();
