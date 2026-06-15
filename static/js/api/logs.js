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

export async function getLogs(projectId, limit) {
  const params = limit ? `?limit=${limit}` : "";
  return request("GET", `/projects/${projectId}/logs${params}`);
}

export async function clearLogs(projectId) {
  return request("DELETE", `/projects/${projectId}/logs`);
}
