export const baseUrl = 'http://localhost:8080';

export async function getSecure() {
	const res = await fetch(`${baseUrl}/secure`, { credentials: 'same-origin' });
	const secureResponse = await res.json();
	return JSON.stringify(secureResponse.session);
}

export async function getApi(token) {
	const res = await fetch(`${baseUrl}/api`, {
		headers: {
			Authorization: 'Bearer ' + token,
			Accept: 'application/json'
		}
	});
	const data = await res.json();

	return JSON.stringify(data);
}

export async function getSession() {
	const res = await fetch(`${baseUrl}/auth/session`, {
		credentials: 'same-origin'
	});
	const sessionResponse = await res.json();
}

export async function postLogin(username, password) {
	const res = await fetch(`${baseUrl}/auth/login`, {
		method: 'POST',
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ username: username, password: password })
	});
	return await res.json();
}

export async function getLogout(username, password) {
	const res = await fetch(`${baseUrl}/auth/logout`, {
		credentials: 'same-origin'
	});

	const logoutResponse = await res.json();
	if (logoutResponse.result == 'error') {
		// may want to return an error here
	}
}
