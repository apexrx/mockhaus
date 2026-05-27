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

export async function getProjects() {
  return request("GET", "/projects");
}

export async function createProject(name) {
  return request("POST", "/projects", { name });
}

export async function deleteProject(id) {
  return request("DELETE", `/projects/${id}`);
}
