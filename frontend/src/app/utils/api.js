import process from "process";

export function get(path, onSuccess, onError) {
    const apiUrl = `http://${process.env.apiHost}/${path}`;
    fetch(apiUrl)
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