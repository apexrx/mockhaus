import { getState, setState, subscribe } from "../utils/state.js";
import { getProjects } from "../api/projects.js";

export function render(state) {
  const projectList = document.getElementById("project-list");
  if (!projectList) return;
  projectList.innerHTML = "";
  (state.projects || []).forEach((project) => {
    const li = document.createElement("li");
    li.textContent = project.name;
    projectList.appendChild(li);
  });
}

subscribe(render);

async function init() {
  const projects = await getProjects();
  setState({ projects });
}

init();
