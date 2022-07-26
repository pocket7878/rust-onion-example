# rust-layered-example

Rust onion architecture application example.

## Layers

Separate each layer into crates:

- domain
- infra
- use_case
- presentation

### Domain

- Define domain models.
- Define repository traits.

### Infra

- Implement repository and other technical details.
- Provide repository implementation instance without exposing real type.

### UseCase

- Implement application logics using domain model interface.
- Define a data transfer object to hide domain model objects from external layers.

### Presentation

- Convert request from out-side world into use_case parameter.
- Convert use-case response into out-side world format.

## Running

```shell
$ cargo run&

$ http localhost:8080/tasks
HTTP/1.1 200 OK
content-length: 2
content-type: application/json
date: Tue, 26 Jul 2022 12:47:37 GMT

[]

$ http POST localhost:8080/tasks name="Hello" due_date="2006-04-13T14:12:53.4242+05:30"
HTTP/1.1 200 OK
content-length: 86
content-type: application/json
date: Tue, 26 Jul 2022 12:47:46 GMT

{
    "due_date": "2006-04-13T14:12:53.4242+05:30",
    "id": 1,
    "name": "Hello",
    "postpone_count": 0
}

$ http PATCH localhost:8080/tasks/1 name="Hello, world"
HTTP/1.1 200 OK
content-length: 93
content-type: application/json
date: Tue, 26 Jul 2022 12:47:56 GMT

{
    "due_date": "2006-04-13T14:12:53.4242+05:30",
    "id": 1,
    "name": "Hello, world",
    "postpone_count": 0
}

$ http localhost:8080/tasks
HTTP/1.1 200 OK
content-length: 95
content-type: application/json
date: Tue, 26 Jul 2022 12:47:59 GMT

[
    {
        "due_date": "2006-04-13T14:12:53.4242+05:30",
        "id": 1,
        "name": "Hello, world",
        "postpone_count": 0
    }
]

$ http PATCH localhost:8080/tasks/1/postpone
HTTP/1.1 200 OK
content-length: 93
content-type: application/json
date: Tue, 26 Jul 2022 12:48:04 GMT

{
    "due_date": "2006-04-20T14:12:53.4242+05:30",
    "id": 1,
    "name": "Hello, world",
    "postpone_count": 1
}

$ http PATCH localhost:8080/tasks/1/postpone
HTTP/1.1 200 OK
content-length: 93
content-type: application/json
date: Tue, 26 Jul 2022 12:48:10 GMT

{
    "due_date": "2006-04-27T14:12:53.4242+05:30",
    "id": 1,
    "name": "Hello, world",
    "postpone_count": 2
}

$ http PATCH localhost:8080/tasks/1/postpone
HTTP/1.1 200 OK
content-length: 93
content-type: application/json
date: Tue, 26 Jul 2022 12:48:11 GMT

{
    "due_date": "2006-05-04T14:12:53.4242+05:30",
    "id": 1,
    "name": "Hello, world",
    "postpone_count": 3
}

$ http PATCH localhost:8080/tasks/1/postpone
HTTP/1.1 422 Unprocessable Entity
content-length: 138
content-type: application/json
date: Tue, 26 Jul 2022 12:48:12 GMT

{
    "api_error": {
        "details": [
            "task can not be postponed more than 3 times"
        ],
        "status": "422 Unprocessable Entity",
        "type": "UnprocessableEntity"
    }
}
```
