import process from "process";

export function get(path, token, onSuccess, onError) {
    const apiUrl = `http://${process.env.apiHost}/${path}`;
    const headers = token ? { "X-API-Key": token } : {};
    console.log(apiUrl);
    console.log(headers);
    fetch(apiUrl, { headers: new Headers(headers) })
        .then((response) => {
            if (!response.ok) {
                onError();
            }
            return response.json()
        })
        .then((data) => {
            onSuccess(data);
        })
        .catch((error) => {
            onError(error);
        })
}