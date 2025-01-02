import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
  vus: 150,
  duration: '600s',
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<500'],
  },
};

export default function () {
  const endpoints = [
    { url: 'http://localhost:3000/categories', weight: 5 },
    { url: 'http://localhost:3000/challenges', weight: 3 }
  ];

  const totalWeight = endpoints.reduce((sum, e) => sum + e.weight, 0);
  let choice = Math.random() * totalWeight;
  let selectedEndpoint;

  for (let endpoint of endpoints) {
    choice -= endpoint.weight;
    if (choice <= 0) {
      selectedEndpoint = endpoint.url;
      break;
    }
  }

  http.get(selectedEndpoint);
  sleep(0.1 + Math.random() * 0.2);
}
