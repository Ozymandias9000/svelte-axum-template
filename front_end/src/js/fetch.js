export const baseUrl = "http://localhost:8080";

export async function getSecure() {
  let res = await fetch(`${baseUrl}/secure`, { credentials: "same-origin" });
  let secureResponse = await res.json();
  return JSON.stringify(secureResponse.session);
}

export async function getApi(api_token) {
  let res = await fetch(`${baseUrl}/api`, {
    headers: {
      Authorization: "Bearer " + api_token,
      Accept: "application/json",
    },
  });
  return await res.json();
}
