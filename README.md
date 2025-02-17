# rudo

`rudo` is a command line and HTTP server for managing tasks

## CLI Usage

Add Task

```bash
$ rudo add "Some boring task"
```

List tasks

```bash
$ rudo list
1 [ ] Some bording task
```
Mark task as done

```bash
$ rudo do 1
```

Mark task as not done

```bash
$ rudo undo 1
```

Delete task

```bash
$ rudo delete 1
```

## HTTP Server Usage

Add Task

```bash
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d "{'title': 'Some boring task'}" \
  http://localhost:8080
```

List tasks

```bash
$ curl http://localhost:8080
{
    "id": 1,
    "title": "Some boring task",
    "done": false
}
```

Mark task as done

```bash
$ curl -X POST http://localhost:8080/do/1
{
    "id": 1,
    "title": "Some boring task",
    "done": true
}
```

Mark task as not done

```bash
$ curl -X POST http://localhost:8080/undo/1
{
    "id": 1,
    "title": "Some boring task",
    "done": false
}
```

Delete task

```bash
$ curl -X POST http://localhost:8080/delete/1
```

