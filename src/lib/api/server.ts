export async function testServerConnection(
  host: string,
  port: number,
): Promise<boolean> {
  try {
    const res = await fetchWithTimeout(
      `http://${host}:${port}/server/details`,
      {},
      100,
    );
    if (!res.ok) return false;
    const data = await res.json();
    if (data.identifier !== "TILEPAD_CONTROLLER_SERVER") return false;
    return true;
  } catch (err) {
    return false;
  }
}

function fetchWithTimeout(url: string, options = {}, timeout = 5000) {
  const controller = new AbortController();
  const signal = controller.signal;

  // Start a timeout to abort the request
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  return fetch(url, { ...options, signal })
    .then((response) => {
      clearTimeout(timeoutId); // Clear timeout if request succeeds
      return response;
    })
    .catch((error) => {
      if (error.name === "AbortError") {
        throw new Error(`Request timed out after ${timeout}ms`);
      }
      throw error;
    });
}
