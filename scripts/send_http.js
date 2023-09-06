import http from 'k6/http';

export const options = {
    vus: 10,
    duration: '30s',
};

export default function () {
    const url = 'http://127.0.0.1:8686/foo';
    const payload = 'ping';

    const params = {
        headers: {
            'Content-Type': 'plain/text',
        },
    };

    http.post(url, payload, params);
}
