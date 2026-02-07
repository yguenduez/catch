GET {{target}}/kv/key1
HTTP 404

POST {{target}}/kv/key1
HTTP 200

GET {{target}}/kv/key1
HTTP 200
Content-Type: application/json

{"value1": "value1"}
