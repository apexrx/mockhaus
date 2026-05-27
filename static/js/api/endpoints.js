const BASE = "/admin";

async function request(method, path, body) {
  const res = await fetch(`${BASE}${path}`, {
    method,
    headers: body ? { "Content-Type": "application/json" } : {},
    body: body ? JSON.stringify(body) : undefined,
  });

  if (!res.ok) {
    // If the Rust server returns our AppError, it comes back as { "error": "..." }
    const errData = await res.json().catch(() => ({}));
    throw new Error(
      errData.error || `${method} ${path} failed with ${res.status}`,
    );
  }

  // 204 No Content has no body, so we can't parse JSON
  if (res.status === 204) return null;

  return res.json();
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
