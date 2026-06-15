const BASE = "/admin";

async function request(method, path, body) {
  const res = await fetch(`${BASE}${path}`, {
    method,
    headers: body ? { 'Content-Type': 'application/json' } : {},
    body: body ? JSON.stringify(body) : undefined,
  });
  if (!res.ok) {
    let errData = {};
    try { errData = await res.json(); } catch (e) {}
    throw new Error(errData.error || `${method} ${path} failed with ${res.status}`);
  }
  // Explicitly ignore responses that we know don't have bodies
  if (res.status === 204 || res.status === 201) return null;
  const text = await res.text();
  if (!text || text.trim() === "") return null;
  return JSON.parse(text);
}

export async function getProjects() {
  return request("GET", "/projects");
}

export async function createProject(name) {
  return request("POST", "/projects", { name });
}

export async function deleteProject(id) {
  return request("DELETE", `/projects/${id}`);
}

export async function getProject(id) {
  return request('GET', `/projects/${id}`);
}
