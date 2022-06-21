# Readme

This is a reproduction of an issue in okapi swagger generation.

In `main.rs` line 37 it is specified that the provided api requires data format of type `text/plain`.

```
#[put("/<key>", data = "<data>", format = "text/plain")]
```

The swagger generation does not obey this annotation and instead defaults to `application/octet-stream`.
If the endpoint is called via incorrectly generated swagger, the webserver will throw an 404 error:

```
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>404 Not Found</title>
</head>
<body align="center">
    <div role="main" align="center">
        <h1>404: Not Found</h1>
        <p>The requested resource could not be found.</p>
        <hr />
    </div>
    <div role="contentinfo" align="center">
        <small>Rocket</small>
    </div>
</body>
</html>
```
