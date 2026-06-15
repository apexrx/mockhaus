import morphdom from "morphdom";

import { getState, setState, subscribe } from "../utils/state.js";
import { getProjects, createProject } from "../api/projects.js";
import { copyText } from "../utils/clipboard.js";
import { showToast } from "../components/toast.js";
import { showPrompt } from "../utils/modal.js";

export function render(state) {
  const projectList = document.getElementById("project-list");

  const tempContainer = document.createElement("div");

  tempContainer.innerHTML = `
    <ul id="project-list" style="list-style: none; padding: 0; margin-top: 2rem;">
      ${(state.projects || []).length === 0
        ? '<li class="muted">No projects found. Create one!</li>'
        : (state.projects || []).map((p) => `
          <li class="project-card">
            <div>
              <h3>${p.name}</h3>
              <span class="muted" style="font-size: 0.8rem;">ID: ${p.id}</span>
              <button class="copy-btn" data-copy-text="${p.id}" title="Copy Project ID">Copy ID</button>
            </div>
            <div class="project-actions">
              <a href="/projects/${p.id}/editor">Editor</a>
              <a href="/projects/${p.id}/inspector">Logs</a>
            </div>
          </li>
        `).join("")
      }
    </ul>
  `;

  morphdom(projectList, tempContainer.firstElementChild);
}

subscribe(render);

async function init() {
  const projects = await getProjects();
  setState({ projects });

  document.getElementById("new-project-btn").addEventListener("click", async () => {
    const name = await showPrompt("Project name:");
    if (!name) return;

    try {
      await createProject(name);
      const projects = await getProjects();
      setState({ projects });
    } catch (err) {
      showToast(err.message, "error");
    }
  });

  document.getElementById("project-list").addEventListener("click", (e) => {
    const copyBtn = e.target.closest(".copy-btn");
    if (copyBtn) {
      e.stopImmediatePropagation();
      copyText(copyBtn.dataset.copyText).then(ok => {
        if (ok) showToast("Project ID copied to clipboard!", "success");
      });
    }
  });
}

init();
