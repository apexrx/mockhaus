import morphdom from "morphdom";
import { getState, setState, subscribe } from "../utils/state.js";
import { getEndpoints, updateEndpoint, addEndpoint } from "../api/endpoints.js";
import { getProject } from "../api/projects.js";
import { showToast } from "../components/toast.js";
import { copyText } from "../utils/clipboard.js";
import { showPrompt } from "../utils/modal.js";

const METHODS = ["GET", "POST", "PUT", "DELETE", "PATCH"];

export function render(state) {
  const list = document.getElementById("endpoint-list");
  const form = document.getElementById("endpoint-form");

  const listContainer = document.createElement("div");
  listContainer.innerHTML = `
    <ul id="endpoint-list">
      ${(state.endpoints || []).length === 0
        ? '<li class="muted">No endpoints yet</li>'
        : (state.endpoints || []).map(ep => `
          <li data-endpoint-id="${ep.id}" class="${state.currentEndpoint?.id === ep.id ? "active" : ""}">
            <span class="method">${ep.method}</span>
            <span class="path">${ep.path}</span>
            <button class="copy-btn" data-url="http://localhost:7070/mock/${state.projectId}${ep.path}" title="Copy URL">Copy</button>
          </li>
        `).join("")
      }
    </ul>
  `;
  morphdom(list, listContainer.firstElementChild);

  const ep = state.currentEndpoint;
  const formContainer = document.createElement("div");
  if (ep) {
    formContainer.innerHTML = `
      <form id="endpoint-form">
        <label>Method
          <select name="method">
            ${METHODS.map(m => `<option value="${m}" ${ep.method === m ? "selected" : ""}>${m}</option>`).join("")}
          </select>
        </label>
        <label>Path
          <input name="path" value="${ep.path}" />
        </label>
        <label>Status Code
          <input name="status_code" type="number" value="${ep.status_code}" />
        </label>
        <label>Response Body
          <textarea name="response_body">${ep.response_body}</textarea>
        </label>
        <button type="submit">Save</button>
      </form>
    `;
    morphdom(form, formContainer.firstElementChild, {
      onBeforeElUpdated(fromEl, toEl) {
        if (fromEl.tagName === "INPUT" || fromEl.tagName === "SELECT" || fromEl.tagName === "TEXTAREA") {
          fromEl.value = toEl.value;
        }
        return true;
      },
    });
  } else {
    formContainer.innerHTML = `<form id="endpoint-form"><p class="muted">Select an endpoint</p></form>`;
    morphdom(form, formContainer.firstElementChild);
  }
}

subscribe(render);

let saving = false;

async function init() {
  const layout = document.getElementById("editor-layout");
  const projectId = layout.dataset.projectId;
  if (!projectId) return;

  try {
    const project = await getProject(projectId);
    document.getElementById("project-name-display").textContent = project.name;
    document.title = `${project.name} // Editor`;
  } catch (err) {
    document.getElementById("project-name-display").textContent = "Unknown Project";
  }

  const endpoints = await getEndpoints(projectId);
  setState({ projectId, endpoints, currentEndpoint: null });

  document.getElementById("new-endpoint-btn").addEventListener("click", async () => {
    const path = await showPrompt("Enter endpoint path (e.g., /api/users):", "/");
    if (!path) return;
    try {
      await addEndpoint(projectId, {
        method: "GET",
        path: path,
        status_code: 200,
        response_body: '{\n  "message": "success"\n}',
      });
      const endpoints = await getEndpoints(projectId);
      setState({ endpoints });
      showToast("Endpoint created", "success");
    } catch (err) {
      showToast(err.message, "error");
    }
  });

  layout.addEventListener("click", (e) => {
    const copyBtn = e.target.closest(".copy-btn");
    if (copyBtn) {
      e.stopImmediatePropagation();
      copyText(copyBtn.dataset.url).then(ok => {
        if (ok) showToast("URL Copied to clipboard!", "success");
      });
      return;
    }

    const item = e.target.closest("[data-endpoint-id]");
    if (item) {
      const id = item.dataset.endpointId;
      const ep = getState().endpoints.find(e => e.id === id);
      if (ep) setState({ currentEndpoint: ep });
    }
  });

  document.getElementById("endpoint-form").addEventListener("submit", async (e) => {
    e.preventDefault();
    if (saving) return;
    saving = true;

    const s = getState();
    const ep = s.currentEndpoint;
    if (!ep) return;

    const form = e.currentTarget;
    const updated = {
      ...ep,
      method: form.querySelector("[name=method]").value,
      path: form.querySelector("[name=path]").value,
      status_code: parseInt(form.querySelector("[name=status_code]").value, 10),
      response_body: form.querySelector("[name=response_body]").value,
    };

    const prev = s.endpoints.find(e => e.id === ep.id);
    const newEndpoints = s.endpoints.map(e => e.id === ep.id ? updated : e);
    setState({ endpoints: newEndpoints, currentEndpoint: updated });

    try {
      await updateEndpoint(s.projectId, ep.id, {
        method: updated.method,
        path: updated.path,
        status_code: updated.status_code,
        response_body: updated.response_body,
      });
    } catch (err) {
      const rollback = getState().endpoints.map(e => e.id === ep.id ? prev : e);
      const curr = getState().currentEndpoint;
      setState({
        endpoints: rollback,
        currentEndpoint: curr?.id === ep.id ? prev : curr,
      });
      showToast(err.message, "error");
    } finally {
      saving = false;
    }
  });
}

init();
