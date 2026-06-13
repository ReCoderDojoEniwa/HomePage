const header = document.getElementById("header");
let lastScrollY = window.scrollY;

window.addEventListener("scroll", () => {
  const currentScrollY = window.scrollY;

  if (currentScrollY > lastScrollY && currentScrollY > 60) {
    header.classList.add("is-hidden");
  } else {
    header.classList.remove("is-hidden");
  }
  lastScrollY = currentScrollY;
});
