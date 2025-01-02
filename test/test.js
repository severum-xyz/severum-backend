import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 500 },
    { duration: '1m', target: 2000 },
    { duration: '1m', target: 5000 },
    { duration: '30s', target: 0 },
  ],
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
