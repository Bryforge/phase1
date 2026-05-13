// Phase1 mobile nav hardening.
// This small fallback keeps the hamburger menu working even if older cached CSS/JS
// leaves the menu hidden on mobile.
(function () {
  function ready(fn) {
    if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", fn, { once: true });
    else fn();
  }

  ready(function () {
    var toggle = document.querySelector(".nav-toggle");
    var links = document.getElementById("nav-links");
    if (!toggle || !links) return;

    function setOpen(open) {
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
      links.classList.toggle("is-open", open);
      document.body.classList.toggle("phase1-nav-open", open);
    }

    function toggleOpen(event) {
      if (event) {
        event.preventDefault();
        event.stopPropagation();
      }
      setOpen(toggle.getAttribute("aria-expanded") !== "true");
    }

    toggle.addEventListener("click", toggleOpen);
    toggle.addEventListener("touchend", toggleOpen, { passive: false });

    links.addEventListener("click", function (event) {
      if (event.target && event.target.closest && event.target.closest("a")) setOpen(false);
    });

    document.addEventListener("keydown", function (event) {
      if (event.key === "Escape") setOpen(false);
    });

    document.addEventListener("click", function (event) {
      if (!document.body.classList.contains("phase1-nav-open")) return;
      if (event.target && event.target.closest && event.target.closest(".nav")) return;
      setOpen(false);
    });
  });
})();
