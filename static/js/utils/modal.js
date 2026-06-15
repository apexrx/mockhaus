export function showPrompt(message, defaultValue = "") {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");
    overlay.className = "modal-overlay";
    overlay.innerHTML = `
      <div class="modal">
        <h3>${message}</h3>
        <input type="text" id="modal-input" value="${defaultValue}" />
        <div class="modal-actions">
          <button class="btn" id="modal-cancel" style="background: #333; color: #fff;">Cancel</button>
          <button class="btn" id="modal-submit">Confirm</button>
        </div>
      </div>
    `;
    document.body.appendChild(overlay);
    const input = document.getElementById("modal-input");
    input.focus();
    input.setSelectionRange(input.value.length, input.value.length);

    const cleanup = (val) => {
      overlay.remove();
      resolve(val);
    };

    document.getElementById("modal-submit").onclick = () => cleanup(input.value);
    document.getElementById("modal-cancel").onclick = () => cleanup(null);
    input.onkeydown = (e) => {
      if (e.key === "Enter") cleanup(input.value);
      if (e.key === "Escape") cleanup(null);
    };
  });
}
