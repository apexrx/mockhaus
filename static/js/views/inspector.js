import morphdom from "morphdom";
import { setState, subscribe } from "../utils/state.js";
import { getLogs } from "../api/logs.js";
import { getProject } from "../api/projects.js";

const renderedIds = new Set();

function fmtTime(unix) {
  return new Date(unix * 1000).toLocaleString();
}

function renderRow(log) {
  const tr = document.createElement("tr");
  tr.innerHTML = `<td>${log.method}</td><td>${log.path}</td><td>${fmtTime(log.received_at)}</td>`;
  return tr;
}

export function render(state) {
  const tbody = document.getElementById("log-body");
  const logs = state.logs || [];

  if (renderedIds.size === 0) {
    const tmp = document.createElement("table");
    tmp.innerHTML = `<tbody id="log-body">${
      logs.length === 0
        ? '<tr><td colspan="3" class="muted">No logs yet</td></tr>'
        : logs.map(l => `<tr><td>${l.method}</td><td>${l.path}</td><td>${fmtTime(l.received_at)}</td></tr>`).join("")
    }</tbody>`;
    morphdom(tbody, tmp.firstElementChild);
    logs.forEach(l => renderedIds.add(l.id));
    return;
  }

  const newLogs = logs.filter(l => !renderedIds.has(l.id));
  if (newLogs.length === 0) return;

  const frag = document.createDocumentFragment();
  for (const log of newLogs) {
    frag.appendChild(renderRow(log));
    renderedIds.add(log.id);
  }
  tbody.prepend(frag);
}

subscribe(render);

async function poll(projectId) {
  try {
    const logs = await getLogs(projectId);
    console.log("Raw logs from server:", logs);
    setState({ logs });
    document.getElementById("inspector-status").innerHTML = `<span style="color: #00cc00;">●</span> Live polling active`;
  } catch (err) {
    console.error("Polling crashed:", err);
    document.getElementById("inspector-status").innerText = `Error: ${err.message}`;
  }
}

async function init() {
  const layout = document.getElementById("inspector");
  const projectId = layout.dataset.projectId;
  if (!projectId) return;

  try {
    const project = await getProject(projectId);
    document.getElementById("project-name-display").textContent = project.name;
    document.title = `${project.name} // Logs`;
  } catch (err) {
    document.getElementById("project-name-display").textContent = "Unknown Project";
  }

  await poll(projectId);

  let id = setInterval(() => poll(projectId), 2000);

  document.addEventListener("visibilitychange", () => {
    if (document.hidden) {
      clearInterval(id);
    } else {
      poll(projectId);
      id = setInterval(() => poll(projectId), 2000);
    }
  });
}

init();
