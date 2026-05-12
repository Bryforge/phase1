document.addEventListener("click", (event) => {
  const link = event.target.closest("a[href]");
  if (!link) return;

  const href = link.getAttribute("href");
  if (!href || href.startsWith("#")) return;

  // Some in-app webviews can swallow animated-layer taps. This fallback keeps
  // external buttons reliable while preserving normal browser behavior first.
  if (link.target === "_blank") {
    event.preventDefault();
    window.open(href, "_blank", "noopener,noreferrer");
  }
});
