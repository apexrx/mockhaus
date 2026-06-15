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

export async function getEndpoints(projectId) {
  return request("GET", `/projects/${projectId}/endpoints`);
}

export async function addEndpoint(projectId, payload) {
  return request("POST", `/projects/${projectId}/endpoints`, payload);
}

export async function updateEndpoint(projectId, endpointId, payload) {
  return request("PUT", `/projects/${projectId}/endpoints/${endpointId}`, payload);
}

export async function deleteEndpoint(projectId, endpointId) {
  return request("DELETE", `/projects/${projectId}/endpoints/${endpointId}`);
}
