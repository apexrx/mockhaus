export function showToast(message, type = "error") {
  const el = document.createElement("div");
  el.className = `toast toast-${type}`;
  el.textContent = message;
  document.body.appendChild(el);
  el.getBoundingClientRect();
  el.classList.add("active");
  setTimeout(() => {
    el.classList.remove("active");
    setTimeout(() => el.remove(), 300);
  }, 3500);
}
